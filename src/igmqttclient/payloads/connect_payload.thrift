# Payload for CONNECT packet
struct ConnectPayload {
    1: string clientIdentifier
    4: ClientInfo clientInfo
    5: string password
    10: map<string, string> appSpecificInfo,
}

struct ClientInfo {
    1: i64 userId,
    2: string userAgent,
    3: i64 clientCapabilities,
    4: i64 endpointCapabilities,
    5: i32 publishFormat,
    6: bool noAutomaticForeground,
    7: bool makeUserAvailableInForeground,
    8: string deviceId,
    9: bool isInitiallyForeground,
    10: i32 networkType,
    11: i32 networkSubtype,
    12: i64 clientMqttSessionId,
    14: list<i32> subscribeTopics,
    15: string clientType,
    16: i64 appId,
    20: string deviceSecret,
    21: byte clientStack,
}
