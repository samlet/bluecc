use crate::models::security_types::*;

#[derive(Deserialize, Debug)]
pub struct SeedRecords {
    pub items: Vec<SeedTypes>
}

#[derive(Deserialize, Debug)]
pub enum SeedTypes {
    X509IssuerProvision(X509IssuerProvision),
    UserLogin(UserLogin),
    UserLoginPasswordHistory(UserLoginPasswordHistory),
    UserLoginHistory(UserLoginHistory),
    UserLoginSession(UserLoginSession),
    SecurityGroup(SecurityGroup),
    SecurityGroupPermission(SecurityGroupPermission),
    SecurityPermission(SecurityPermission),
    UserLoginSecurityGroup(UserLoginSecurityGroup),
    ProtectedView(ProtectedView),
    TarpittedLoginView(TarpittedLoginView),
}
