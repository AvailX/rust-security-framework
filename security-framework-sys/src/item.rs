use core_foundation_sys::string::CFStringRef;

extern "C" {
    pub static kSecClass: CFStringRef;
    pub static kSecClassInternetPassword: CFStringRef;
    pub static kSecClassGenericPassword: CFStringRef;
    pub static kSecClassCertificate: CFStringRef;
    pub static kSecClassKey: CFStringRef;
    pub static kSecClassIdentity: CFStringRef;

    pub static kSecMatchLimit: CFStringRef;
    pub static kSecMatchLimitAll: CFStringRef;

    pub static kSecReturnData: CFStringRef;
    pub static kSecReturnAttributes: CFStringRef;
    pub static kSecReturnRef: CFStringRef;
    pub static kSecReturnPersistentRef: CFStringRef;

    pub static kSecMatchSearchList: CFStringRef;

    pub static kSecAttrApplicationLabel: CFStringRef;
    pub static kSecAttrKeyType: CFStringRef;
    pub static kSecAttrLabel: CFStringRef;
    pub static kSecAttrIsPermanent: CFStringRef;
    pub static kSecAttrPublicKeyHash: CFStringRef;
    pub static kSecPrivateKeyAttrs: CFStringRef;
    pub static kSecPublicKeyAttrs: CFStringRef;

    pub static kSecAttrKeyClass: CFStringRef;
    pub static kSecAttrKeyClassPublic: CFStringRef;
    pub static kSecAttrKeyClassPrivate: CFStringRef;
    pub static kSecAttrKeyClassSymmetric: CFStringRef;

    pub static kSecUseKeychain: CFStringRef;
 
    pub static kSecUseDataProtectionKeychain: CFStringRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    pub static kSecAttrTokenID: CFStringRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    pub static kSecAttrTokenIDSecureEnclave: CFStringRef;

    pub static kSecAttrKeySizeInBits: CFStringRef;

    pub static kSecAttrKeyTypeECSECPrimeRandom: CFStringRef;
    pub static kSecAttrKeyTypeRSA: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeDSA: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeAES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeDES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyType3DES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeRC4: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeRC2: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeCAST: CFStringRef;
    #[cfg(any(feature = "OSX_10_9", target_os = "ios"))]
    pub static kSecAttrKeyTypeEC: CFStringRef;

    pub static kSecAttrAccessGroup: CFStringRef;
    pub static kSecAttrAccessGroupToken: CFStringRef;

    pub static kSecAttrAuthenticationType: CFStringRef;
    pub static kSecAttrPath: CFStringRef;
    pub static kSecAttrPort: CFStringRef;
    pub static kSecAttrProtocol: CFStringRef;
    pub static kSecAttrSecurityDomain: CFStringRef;
    pub static kSecAttrServer: CFStringRef;
    pub static kSecAttrService: CFStringRef;
    pub static kSecAttrAccessControl: CFStringRef;
    pub static kSecAttrAccount: CFStringRef;
    pub static kSecValueData: CFStringRef;
    pub static kSecValueRef: CFStringRef;
    pub static kSecAttrAccessibleWhenUnlockedThisDeviceOnly: CFStringRef;
    pub static kSecUseAuthenticationContext: CFStringRef;
    pub static kSecUseAuthenticationUI: CFStringRef;
    pub static kSecUseAuthenticationUISkip: CFStringRef;
    pub static kSecUseOperationPrompt: CFStringRef;
    pub static kSecAttrAccessible: CFStringRef;

}
