

/// A representation of a type of channel.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ChannelType {
    /// An indicator that the channel is the channel of a [`Group`].
    ///
    ///  [`Group`]: struct.Group.html
    Group,
    /// An indicator that the channel is a [`PrivateChannel`].
    ///
    ///  [`PrivateChannel`]: struct.PrivateChannel.html
    Private,
    /// An indicator that the channel is a text [`GuildChannel`].
    ///
    ///  [`GuildChannel`]: struct.GuildChannel.html
    Text,
    /// An indicator that the channel is a voice [`GuildChannel`].
    ///
    ///  [`GuildChannel`]: struct.GuildChannel.html
    Voice,
}

impl ChannelType {
    pub fn name(&self) -> &str {
        match *self {
            ChannelType::Group => "group",

            ChannelType::Private => "private",

            ChannelType::Text => "text",

            ChannelType::Voice => "voice",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "group" => Some(ChannelType::Group),
            "private" => Some(ChannelType::Private),
            "text" => Some(ChannelType::Text),
            "voice" => Some(ChannelType::Voice),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid ChannelType",
                                         Value::String(name)))
    }
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            ChannelType::Group => 0,
            ChannelType::Private => 1,
            ChannelType::Text => 2,
            ChannelType::Voice => 3,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(ChannelType::Group),
            1 => Some(ChannelType::Private),
            2 => Some(ChannelType::Text),
            3 => Some(ChannelType::Voice),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid ChannelType"))
    }
}

/// The type of a user [`Connection`].
/// **Note**: This is not in any way related to a WebSocket connection.
/// 
///   [`Connection`]: struct.Connection.html
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ConnectionType {
    /// A [Battle.net] connection.
    ///
    ///  [Battle.net]: https://battle.net
    BattleNet,
    /// A [Steam] connection.
    ///
    ///  [Steam]: http://steampowered.com
    Steam,
    /// A [Twitch.tv] connection.
    ///
    ///  [Twitch.tv]: https://twitch.tv
    Twitch,
    /// A [YouTube Gaming] connection.
    ///
    ///  [YouTube Gaming]: https://gaming.youtube.com
    YouTube,
}

impl ConnectionType {
    pub fn name(&self) -> &str {
        match *self {
            ConnectionType::BattleNet => "battlenet",

            ConnectionType::Steam => "steam",

            ConnectionType::Twitch => "twitch",

            ConnectionType::YouTube => "youtube",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "battlenet" => Some(ConnectionType::BattleNet),
            "steam" => Some(ConnectionType::Steam),
            "twitch" => Some(ConnectionType::Twitch),
            "youtube" => Some(ConnectionType::YouTube),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid ConnectionType",
                                         Value::String(name)))
    }
}

/// An enum that represents a default avatar. The default avatar is calculated via the result of `discriminator % 5`.
/// The hash of the avatar can be retrieved via calling [`name`] on the enum.
/// 
///   [`name`]: #method.name
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum DefaultAvatar {
    /// The avatar when the result is 0.
    Blurple,
    /// The avatar when the result is 1.
    Grey,
    /// The avatar when the result is 2.
    Green,
    /// The avatar when the result is 3.
    Orange,
    /// The avatar when the result is 4.
    Red,
}

impl DefaultAvatar {
    pub fn name(&self) -> &str {
        match *self {
            DefaultAvatar::Blurple => "6debd47ed13483642cf09e832ed0bc1b",

            DefaultAvatar::Grey => "322c936a8c8be1b803cd94861bdfa868",

            DefaultAvatar::Green => "dd4dbc0016779df1378e7812eabaa04d",

            DefaultAvatar::Orange => "0e291f67c9274a1abdddeb3fd919cbaa",

            DefaultAvatar::Red => "1cbd08c76f8af6dddce02c5138971129",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "6debd47ed13483642cf09e832ed0bc1b" => Some(DefaultAvatar::Blurple),
            "322c936a8c8be1b803cd94861bdfa868" => Some(DefaultAvatar::Grey),
            "dd4dbc0016779df1378e7812eabaa04d" => Some(DefaultAvatar::Green),
            "0e291f67c9274a1abdddeb3fd919cbaa" => Some(DefaultAvatar::Orange),
            "1cbd08c76f8af6dddce02c5138971129" => Some(DefaultAvatar::Red),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid DefaultAvatar",
                                         Value::String(name)))
    }
}

/// A special feature, such as for VIP guilds, that a [`Guild`] has had granted to them.
/// 
///   [`Guild`]: struct.Guild.html
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Feature {
    /// The guild can set a custom [`splash`][`Guild::splash`] image on invite URLs.
    ///
    ///  [`Guild::splash`]: struct.Guild.html#structfield.splash
    InviteSplash,
    /// The guild can set a Vanity URL, which is a custom-named permanent invite URL.
    VanityUrl,
    /// The guild has access to VIP voice channel regions.
    VipRegions,
}

impl Feature {
    pub fn name(&self) -> &str {
        match *self {
            Feature::InviteSplash => "INVITE_SPLASH",

            Feature::VanityUrl => "VANITY_URL",

            Feature::VipRegions => "VIP_REGIONS",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "INVITE_SPLASH" => Some(Feature::InviteSplash),
            "VANITY_URL" => Some(Feature::VanityUrl),
            "VIP_REGIONS" => Some(Feature::VipRegions),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid Feature",
                                         Value::String(name)))
    }
}

/// The type of activity that is being performed when playing a [`Game`].
/// 
///   [`Game`]: struct.Game.html
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameType {
    /// An indicator that a person is playing a game.
    Playing,
    /// An indicator that a person is streaming something to a service.
    Streaming,
}

impl GameType {
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            GameType::Playing => 0,
            GameType::Streaming => 1,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(GameType::Playing),
            1 => Some(GameType::Streaming),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid GameType"))
    }
}

/// The type of status update during a [service incident][`Incident`].
/// 
///   [`Incident`]: struct.Incident.html
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum IncidentStatus {
    /// 
    Identified,
    /// 
    Investigating,
    /// 
    Monitoring,
    /// 
    Postmortem,
    /// 
    Resolved,
}

impl IncidentStatus {
    pub fn name(&self) -> &str {
        match *self {
            IncidentStatus::Identified => "identified",

            IncidentStatus::Investigating => "investigating",

            IncidentStatus::Monitoring => "monitoring",

            IncidentStatus::Postmortem => "postmortem",

            IncidentStatus::Resolved => "resolved",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "identified" => Some(IncidentStatus::Identified),
            "investigating" => Some(IncidentStatus::Investigating),
            "monitoring" => Some(IncidentStatus::Monitoring),
            "postmortem" => Some(IncidentStatus::Postmortem),
            "resolved" => Some(IncidentStatus::Resolved),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid IncidentStatus",
                                         Value::String(name)))
    }
}

/// Allows messages to be differentiated between regular ones and system ones like pins and group call updates.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum MessageType {
    /// 
    Regular,
    /// 
    GroupRecipientAddition,
    /// 
    GroupRecipientRemoval,
    /// 
    GroupCallCreation,
    /// 
    GroupNameUpdate,
    /// 
    GroupIconUpdate,
    /// 
    PinsAdd,
}

impl MessageType {
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            MessageType::Regular => 0,
            MessageType::GroupRecipientAddition => 1,
            MessageType::GroupRecipientRemoval => 2,
            MessageType::GroupCallCreation => 3,
            MessageType::GroupNameUpdate => 4,
            MessageType::GroupIconUpdate => 5,
            MessageType::PinsAdd => 6,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(MessageType::Regular),
            1 => Some(MessageType::GroupRecipientAddition),
            2 => Some(MessageType::GroupRecipientRemoval),
            3 => Some(MessageType::GroupCallCreation),
            4 => Some(MessageType::GroupNameUpdate),
            5 => Some(MessageType::GroupIconUpdate),
            6 => Some(MessageType::PinsAdd),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid MessageType"))
    }
}

/// Identifier for the notification level of a channel.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum NotificationLevel {
    /// 
    All,
    /// 
    Mentions,
    /// 
    Nothing,
    /// 
    Parent,
}

impl NotificationLevel {
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            NotificationLevel::All => 0,
            NotificationLevel::Mentions => 1,
            NotificationLevel::Nothing => 2,
            NotificationLevel::Parent => 3,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(NotificationLevel::All),
            1 => Some(NotificationLevel::Mentions),
            2 => Some(NotificationLevel::Nothing),
            3 => Some(NotificationLevel::Parent),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid NotificationLevel"))
    }
}

/// The representation of a user's status like online, idle, etc.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum OnlineStatus {
    /// 
    DoNotDisturb,
    /// 
    Idle,
    /// 
    Invisible,
    /// 
    Offline,
    /// 
    Online,
}

impl OnlineStatus {
    pub fn name(&self) -> &str {
        match *self {
            OnlineStatus::DoNotDisturb => "dnd",

            OnlineStatus::Idle => "idle",

            OnlineStatus::Invisible => "invisible",

            OnlineStatus::Offline => "offline",

            OnlineStatus::Online => "online",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "dnd" => Some(OnlineStatus::DoNotDisturb),
            "idle" => Some(OnlineStatus::Idle),
            "invisible" => Some(OnlineStatus::Invisible),
            "offline" => Some(OnlineStatus::Offline),
            "online" => Some(OnlineStatus::Online),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid OnlineStatus",
                                         Value::String(name)))
    }
}

/// The name of a region that a guild's voice server can be located in.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Region {
    /// 
    Amsterdam,
    /// 
    Brazil,
    /// 
    EuCentral,
    /// 
    EuWest,
    /// 
    Frankfurt,
    /// 
    London,
    /// 
    Syndey,
    /// 
    UsCentral,
    /// 
    UsEast,
    /// 
    UsSouth,
    /// 
    UsWest,
    /// 
    VipAmsterdam,
    /// 
    VipUsEast,
    /// 
    VipUsWest,
}

impl Region {
    pub fn name(&self) -> &str {
        match *self {
            Region::Amsterdam => "amsterdam",

            Region::Brazil => "brazil",

            Region::EuCentral => "eu-central",

            Region::EuWest => "eu-west",

            Region::Frankfurt => "frankfurt",

            Region::London => "london",

            Region::Syndey => "sydney",

            Region::UsCentral => "us-central",

            Region::UsEast => "us-east",

            Region::UsSouth => "us-south",

            Region::UsWest => "us-west",

            Region::VipAmsterdam => "vip-amsterdam",

            Region::VipUsEast => "vip-us-east",

            Region::VipUsWest => "vip-us-west",

        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "amsterdam" => Some(Region::Amsterdam),
            "brazil" => Some(Region::Brazil),
            "eu-central" => Some(Region::EuCentral),
            "eu-west" => Some(Region::EuWest),
            "frankfurt" => Some(Region::Frankfurt),
            "london" => Some(Region::London),
            "sydney" => Some(Region::Syndey),
            "us-central" => Some(Region::UsCentral),
            "us-east" => Some(Region::UsEast),
            "us-south" => Some(Region::UsSouth),
            "us-west" => Some(Region::UsWest),
            "vip-amsterdam" => Some(Region::VipAmsterdam),
            "vip-us-east" => Some(Region::VipUsEast),
            "vip-us-west" => Some(Region::VipUsWest),
            _ => None,
        }
    }

    pub fn decode_str(value: Value) -> Result<Self> {
        let name = try!(into_string(value));

        Self::from_str(&name)
            .ok_or_else(|| Error::Decode("Expected valid Region",
                                         Value::String(name)))
    }
}

/// The type of a relationship between two users.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum RelationshipType {
    /// When one user blocked the other one.
    Blocked,
    /// When users are friends.
    Friends,
    /// When an incoming friend request was received.
    IncomingRequest,
    /// When a friend request was ignored.
    Ignored,
    /// When an outgoing friend request was sent.
    OutgoingRequest,
}

impl RelationshipType {
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            RelationshipType::Blocked => 0,
            RelationshipType::Friends => 1,
            RelationshipType::IncomingRequest => 2,
            RelationshipType::Ignored => 3,
            RelationshipType::OutgoingRequest => 4,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(RelationshipType::Blocked),
            1 => Some(RelationshipType::Friends),
            2 => Some(RelationshipType::IncomingRequest),
            3 => Some(RelationshipType::Ignored),
            4 => Some(RelationshipType::OutgoingRequest),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid RelationshipType"))
    }
}

/// The level to set as criteria prior to a user being able to send messages in a [`Guild`].
/// 
///   [`Guild`]: struct.Guild.html
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum VerificationLevel {
    /// Requires a verified e-mail and being registered for more than 10 minutes. This is stylised as tableflip in the client.
    High,
    /// Requires a verified e-mail.
    Low,
    /// Requires a verified e-mail and being registered for more than 5 minutes.
    Medium,
    /// No verification.
    None,
}

impl VerificationLevel {
    #[allow(dead_code)]
    pub fn num(&self) -> u64 {
        match *self {
            VerificationLevel::High => 0,
            VerificationLevel::Low => 1,
            VerificationLevel::Medium => 2,
            VerificationLevel::None => 3,
        }
    }

    #[allow(dead_code)]
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(VerificationLevel::High),
            1 => Some(VerificationLevel::Low),
            2 => Some(VerificationLevel::Medium),
            3 => Some(VerificationLevel::None),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn decode(value: Value) -> Result<Self> {
        into_u64(value)
            .ok()
            .and_then(Self::from_num)
            .ok_or_else(|| Error::Other("Expected valid VerificationLevel"))
    }
}



/// A component that was affected during a service incident.
/// 
/// This is pulled from the Discord status page.
#[derive(Clone, Debug)]
pub struct AffectedComponent {
    /// The name of the affected component.
    pub name: String,
}

impl AffectedComponent {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<AffectedComponent> {
        let mut map = try!(into_map(value));

        Ok(AffectedComponent {
            name: try!(remove(&mut map, "name").and_then(into_string)),
        })
    }
}

/// Information about a user's application. An application does not necessarily have an associated bot user.
#[derive(Clone, Debug)]
pub struct ApplicationInfo {
    /// The bot user associated with this application. See [BotApplication] for more information.
    ///
    ///  [BotApplication]: struct.BotApplication.html
    pub bot: Option<BotApplication>,
    /// Whether or not the bot is public. If a bot is public, anyone may invite it to their [Guild]. While a bot is private, only the owner may add it to a guild.
    ///
    ///  [Guild]: struct.Guild.html
    pub bot_public: bool,
    /// Whether or not the bot requires an OAuth2 code grant.
    pub bot_require_code_grant: bool,
    /// A description of the application, assigned by the application owner.
    pub description: String,
    /// A set of bitflags assigned to the application, which represent gated feature flags that have been enabled for the application's bot user.
    ///If the application does not have a bot user, then bitflags are not present.
    pub flags: Option<u64>,
    /// A hash pointing to the application's icon. This is not necessarily equivalent to the bot user's avatar. If there is no icon assigned, then this is None.
    pub icon: Option<String>,
    /// The numeric id of the application.
    pub id: UserId,
    /// The name assigned to the application by the application owner.
    pub name: String,
    /// A list of redirect URIs assigned to the application.
    pub redirect_uris: Vec<String>,
    /// A list of RPC Origins asigned to the application.
    pub rpc_origins: Vec<String>,
    /// The given secret to the application. Note that this is not equivalent to an application's bot user's token.
    pub secret: String,
}

impl ApplicationInfo {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<ApplicationInfo> {
        let mut map = try!(into_map(value));

        Ok(ApplicationInfo {
            bot: try!(opt(&mut map, "bot", BotApplication::decode)),
            bot_public: req!(try!(remove(&mut map, "bot_public")).as_bool()),
            bot_require_code_grant: req!(try!(remove(&mut map, "bot_require_code_grant")).as_bool()),
            description: try!(remove(&mut map, "description").and_then(into_string)),
            flags: remove(&mut map, "flags").ok().and_then(|v| v.as_u64()),
            icon: try!(opt(&mut map, "icon", into_string)),
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            redirect_uris: try!(remove(&mut map, "redirect_uris").and_then(|v| decode_array(v, into_string))),
            rpc_origins: try!(remove(&mut map, "rpc_origins").and_then(|v| decode_array(v, into_string))),
            secret: try!(remove(&mut map, "secret").and_then(into_string)),
        })
    }
}

/// A file uploaded with a message. Not to be confused with [`Embed`]s.
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct Attachment {
    /// The unique ID given to this attachment.
    pub id: String,
    /// The filename of the file that was uploaded. This is equivalent to what the uploader had their file named.
    pub filename: String,
    /// If the attachment is an image, then the height of the image is provided.
    pub height: Option<u64>,
    /// The proxy URL.
    pub proxy_url: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The URL of the uploaded attachment.
    pub url: String,
    /// If the attachment is an image, then the width of the image is provided.
    pub width: Option<u64>,
}

impl Attachment {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Attachment> {
        let mut map = try!(into_map(value));

        Ok(Attachment {
            id: try!(remove(&mut map, "id").and_then(into_string)),
            filename: try!(remove(&mut map, "filename").and_then(into_string)),
            height: remove(&mut map, "height").ok().and_then(|v| v.as_u64()),
            proxy_url: try!(remove(&mut map, "proxy_url").and_then(into_string)),
            size: req!(try!(remove(&mut map, "size")).as_u64()),
            url: try!(remove(&mut map, "url").and_then(into_string)),
            width: remove(&mut map, "width").ok().and_then(|v| v.as_u64()),
        })
    }
}

/// A representation of a banning of a user.
#[derive(Clone, Debug)]
pub struct Ban {
    /// The reason given for this ban.
    ///
    ///**Note**: Until the Audit Log feature is completed by Discord, then this will always be None.
    pub reason: Option<String>,
    /// The user that was banned.
    pub user: User,
}

impl Ban {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Ban> {
        let mut map = try!(into_map(value));

        Ok(Ban {
            reason: try!(opt(&mut map, "reason", into_string)),
            user: try!(remove(&mut map, "user").and_then(User::decode)),
        })
    }
}

/// Information about an application with an application's bot user.
#[derive(Clone, Debug)]
pub struct BotApplication {
    /// The Id of the bot user.
    pub id: UserId,
    /// A hash of the avatar, if one is assigned. This can be used to generate a full URL.
    pub avatar: Option<String>,
    /// Whether or not this is a bot.
    pub bot: bool,
    /// The discriminator assigned to the user's username. While discriminators are not unique, the username#discriminator combination is.
    pub discriminator: u16,
    /// The username.
    pub name: String,
    /// The token used to login to the bot user.
    ///
    ///**Note**: Keep this information private, as untrusted users can use it to perform any action on a bot.
    pub token: String,
}

impl BotApplication {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<BotApplication> {
        let mut map = try!(into_map(value));

        Ok(BotApplication {
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            avatar: try!(opt(&mut map, "avatar", into_string)),
            bot: try!(opt(&mut map, "bot", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            discriminator: try!(remove(&mut map, "discriminator").and_then(decode_discriminator)),
            name: try!(remove(&mut map, "username").and_then(into_string)),
            token: try!(remove(&mut map, "token").and_then(into_string)),
        })
    }
}

/// A representation of the data retrieved from the bot gateway endpoint.
/// 
/// This is different from the [`Gateway`], as this includes the number of shards that is recommended for use by the bot and can only be used by bots.
/// 
///   [`Gateway`]: struct.Gateway.html
#[derive(Clone, Debug)]
pub struct BotGateway {
    /// The number of shards that is recommended to be used by the bot.
    pub shards: u64,
    /// The gateway to connect to.
    pub url: String,
}

impl BotGateway {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<BotGateway> {
        let mut map = try!(into_map(value));

        Ok(BotGateway {
            shards: req!(try!(remove(&mut map, "shards")).as_u64()),
            url: try!(remove(&mut map, "url").and_then(into_string)),
        })
    }
}

/// An active group or private call. These are different from [voice channel][`ChannelType::Voice`]s in guilds.
/// 
///   [`ChannelType::Voice`]: enum.ChannelType.html#Voice.v
#[derive(Clone, Debug)]
pub struct Call {
    /// The group or private channel that the call is associated with.
    pub channel_id: ChannelId,
    /// The Id of the [message][`Message`] denoting that the call is active.
    ///
    ///  [`Message`]: struct.Message.html
    pub message_id: MessageId,
    /// The [region][`Region`] that the call is taking place in.
    ///
    ///  [`Region`]: enum.Region.html
    pub region: String,
    /// A list of users that are currently being ringed.
    pub ringing: Vec<UserId>,
    /// Whether the server hosting the call is unavailable.
    pub unavailable: bool,
    /// The users present in the call.
    pub voice_states: HashMap<UserId, VoiceState>,
}

impl Call {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Call> {
        let mut map = try!(into_map(value));

        Ok(Call {
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            message_id: try!(remove(&mut map, "message_id").and_then(MessageId::decode)),
            region: try!(remove(&mut map, "region").and_then(into_string)),
            ringing: try!(remove(&mut map, "ringing").and_then(|v| decode_array(v, UserId::decode))),
            unavailable: req!(try!(remove(&mut map, "unavailable")).as_bool()),
            voice_states: try!(remove(&mut map, "voice_states").and_then(decode_voice_states)),
        })
    }
}

/// An override for a [channel][`Channel`].
/// 
///   [`Channel`]: enum.Channel.html
#[derive(Clone, Debug)]
pub struct ChannelOverride {
    /// The channel this override is for.
    pub channel_id: ChannelId,
    /// The notification level to use for the channel.
    pub message_notifications: NotificationLevel,
    /// Whether or not the channel is muted; while this will not show a notification indicator for the channel, it will continue to show when the user is mentioned in it.
    pub muted: bool,
}

impl ChannelOverride {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<ChannelOverride> {
        let mut map = try!(into_map(value));

        Ok(ChannelOverride {
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            message_notifications: try!(remove(&mut map, "message_notifications").and_then(NotificationLevel::decode)),
            muted: req!(try!(remove(&mut map, "muted")).as_bool()),
        })
    }
}

/// Information about the current application and its owner.
#[derive(Clone, Debug)]
pub struct CurrentApplicationInfo {
    /// 
    pub description: String,
    /// 
    pub icon: Option<String>,
    /// 
    pub id: UserId,
    /// 
    pub name: String,
    /// 
    pub owner: User,
    /// 
    pub rpc_origins: Vec<String>,
}

impl CurrentApplicationInfo {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<CurrentApplicationInfo> {
        let mut map = try!(into_map(value));

        Ok(CurrentApplicationInfo {
            description: try!(remove(&mut map, "description").and_then(into_string)),
            icon: try!(opt(&mut map, "icon", into_string)),
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            owner: try!(remove(&mut map, "owner").and_then(User::decode)),
            rpc_origins: try!(opt(&mut map, "rpc_origins", |v| decode_array(v, into_string))).unwrap_or(Vec::default()),
        })
    }
}

/// Information about the current user.
#[derive(Clone, Debug)]
pub struct CurrentUser {
    /// 
    pub id: UserId,
    /// 
    pub avatar: Option<String>,
    /// 
    pub bot: bool,
    /// 
    pub discriminator: u16,
    /// 
    pub email: Option<String>,
    /// 
    pub mfa_enabled: bool,
    /// Whether the current user has logged in on mobile before.
    pub mobile: Option<bool>,
    /// 
    pub name: String,
    /// 
    pub verified: bool,
}

impl CurrentUser {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<CurrentUser> {
        let mut map = try!(into_map(value));

        Ok(CurrentUser {
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            avatar: try!(opt(&mut map, "avatar", into_string)),
            bot: try!(opt(&mut map, "bot", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            discriminator: try!(remove(&mut map, "discriminator").and_then(decode_discriminator)),
            email: try!(opt(&mut map, "email", into_string)),
            mfa_enabled: req!(try!(remove(&mut map, "mfa_enabled")).as_bool()),
            mobile: try!(opt(&mut map, "mobile", |v| Ok(req!(v.as_bool())))),
            name: try!(remove(&mut map, "username").and_then(into_string)),
            verified: req!(try!(remove(&mut map, "verified")).as_bool()),
        })
    }
}

/// Represents a rich embed which allows using richer markdown, multiple fields and more. This was heavily inspired by [slack's attachments].
/// 
/// You can include an attachment in your own message by a user or a bot, or in a webhook.
/// 
/// **Note**: Maximum amount of characters you can put is 256 in a field name, 1024 in a field value, and 2048 in a description.
/// 
///   [slack's attachments]: https://api.slack.com/docs/message-attachments
#[derive(Clone, Debug)]
pub struct Embed {
    /// Author information about the embed.
    pub author: Option<EmbedAuthor>,
    /// The colour code of the embed.
    pub colour: Colour,
    /// The description of the embed. This is the long string of text.
    pub description: Option<String>,
    /// The array of fields of the embed.
    pub fields: Option<Vec<EmbedField>>,
    /// The image information of the embed.
    pub image: Option<EmbedImage>,
    /// The type of the embed. For webhook embeds, this is always `rich`.
    pub kind: String,
    /// The provider information for the embed.
    pub provider: Option<EmbedProvider>,
    /// The thumbnail provided for the embed.
    pub thumbnail: Option<EmbedThumbnail>,
    /// Timestamp of embed content.
    pub timestamp: Option<String>,
    /// The title of the embed.
    pub title: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The embed's video information.
    pub video: Option<EmbedVideo>,
}

impl Embed {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Embed> {
        let mut map = try!(into_map(value));

        Ok(Embed {
            author: try!(opt(&mut map, "author", EmbedAuthor::decode)),
            colour: try!(opt(&mut map, "color", Colour::decode)).unwrap_or(Colour::default()),
            description: try!(opt(&mut map, "description", into_string)),
            fields: try!(opt(&mut map, "fields", |v| decode_array(v, EmbedField::decode))),
            image: try!(opt(&mut map, "image", EmbedImage::decode)),
            kind: try!(remove(&mut map, "type").and_then(into_string)),
            provider: try!(opt(&mut map, "provider", EmbedProvider::decode)),
            thumbnail: try!(opt(&mut map, "thumbnail", EmbedThumbnail::decode)),
            timestamp: try!(opt(&mut map, "timestamp", into_string)),
            title: try!(opt(&mut map, "title", into_string)),
            url: try!(opt(&mut map, "url", into_string)),
            video: try!(opt(&mut map, "video", EmbedVideo::decode)),
        })
    }
}

/// An author object in an [`Embed`].
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct EmbedAuthor {
    /// The URL of the author icon. Note this only supports HTTP(s).
    pub icon_url: Option<String>,
    /// The name of the author.
    pub name: String,
    /// A proxied URL of the author icon.
    pub proxy_icon_url: Option<String>,
    /// The URL of the author.
    pub url: Option<String>,
}

impl EmbedAuthor {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedAuthor> {
        let mut map = try!(into_map(value));

        Ok(EmbedAuthor {
            icon_url: try!(opt(&mut map, "icon_url", into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            proxy_icon_url: try!(opt(&mut map, "proxy_icon_url", into_string)),
            url: try!(opt(&mut map, "url", into_string)),
        })
    }
}

/// A field object in an [`Embed`].
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct EmbedField {
    /// Whether or not the field should display as inline.
    pub inline: bool,
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: String,
}

impl EmbedField {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedField> {
        let mut map = try!(into_map(value));

        Ok(EmbedField {
            inline: req!(try!(remove(&mut map, "inline")).as_bool()),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            value: try!(remove(&mut map, "value").and_then(into_string)),
        })
    }
}

/// Footer information about an [`Embed`].
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct EmbedFooter {
    /// The URL of the footer icon. Note this only supports HTTP(s).
    pub icon_url: String,
    /// A proxied URL of the footer icon.
    pub proxy_icon_url: String,
    /// The associated text with the footer.
    pub text: String,
}

impl EmbedFooter {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedFooter> {
        let mut map = try!(into_map(value));

        Ok(EmbedFooter {
            icon_url: try!(remove(&mut map, "icon_url").and_then(into_string)),
            proxy_icon_url: try!(remove(&mut map, "proxy_icon_url").and_then(into_string)),
            text: try!(remove(&mut map, "text").and_then(into_string)),
        })
    }
}

/// An image object in an [`Embed`].
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct EmbedImage {
    /// The height of the image.
    pub height: u64,
    /// A proxied URL of the image.
    pub proxy_url: String,
    /// Source URL of the image. Note this only supports HTTP(s).
    pub url: String,
    /// The width of the image.
    pub width: u64,
}

impl EmbedImage {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedImage> {
        let mut map = try!(into_map(value));

        Ok(EmbedImage {
            height: req!(try!(remove(&mut map, "height")).as_u64()),
            proxy_url: try!(remove(&mut map, "proxy_url").and_then(into_string)),
            url: try!(remove(&mut map, "url").and_then(into_string)),
            width: req!(try!(remove(&mut map, "width")).as_u64()),
        })
    }
}

/// The provider of an embed.
#[derive(Clone, Debug)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: String,
    /// The URL of the provider.
    pub url: Option<String>,
}

impl EmbedProvider {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedProvider> {
        let mut map = try!(into_map(value));

        Ok(EmbedProvider {
            name: try!(remove(&mut map, "name").and_then(into_string)),
            url: try!(opt(&mut map, "url", into_string)),
        })
    }
}

/// The dimensions and URL of an embed thumbnail.
#[derive(Clone, Debug)]
pub struct EmbedThumbnail {
    /// The height of the thumbnail in pixels.
    pub height: u64,
    /// A proxied URL of the thumbnail.
    pub proxy_url: String,
    /// The source URL of the thumbnail. Note this only supports HTTP(s).
    pub url: String,
    /// The width of the thumbnail in pixels.
    pub width: u64,
}

impl EmbedThumbnail {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedThumbnail> {
        let mut map = try!(into_map(value));

        Ok(EmbedThumbnail {
            height: req!(try!(remove(&mut map, "height")).as_u64()),
            proxy_url: try!(remove(&mut map, "proxy_url").and_then(into_string)),
            url: try!(remove(&mut map, "url").and_then(into_string)),
            width: req!(try!(remove(&mut map, "width")).as_u64()),
        })
    }
}

/// A video information object in an [`Embed`].
/// 
///   [`Embed`]: struct.Embed.html
#[derive(Clone, Debug)]
pub struct EmbedVideo {
    /// The height of the video.
    pub height: u64,
    /// The source URL of the video.
    pub url: String,
    /// The width of the video.
    pub width: u64,
}

impl EmbedVideo {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmbedVideo> {
        let mut map = try!(into_map(value));

        Ok(EmbedVideo {
            height: req!(try!(remove(&mut map, "height")).as_u64()),
            url: try!(remove(&mut map, "url").and_then(into_string)),
            width: req!(try!(remove(&mut map, "width")).as_u64()),
        })
    }
}

/// Represents a custom guild emoji, which can either be created using the API, or via an integration. Emojis created using the API only work within the guild it was created in.
#[derive(Clone, Debug)]
pub struct Emoji {
    /// The Id of the emoji.
    pub id: EmojiId,
    /// The name of the emoji. It must be at least 2 characters long and can only contain alphanumeric characters and underscores.
    pub name: String,
    /// Whether the emoji is managed via an [`Integration`] service.
    ///[`Integration`]: struct.Integration.html
    pub managed: bool,
    /// Whether the emoji name needs to be surrounded by colons in order to be used by the client.
    pub require_colons: bool,
    /// A list of [`Role`]s that are allowed to use the emoji. If there are no roles specified, then usage is unrestricted.
    ///[`Role`]: struct.Role.html
    pub roles: Vec<RoleId>,
}

impl Emoji {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Emoji> {
        let mut map = try!(into_map(value));

        Ok(Emoji {
            id: try!(remove(&mut map, "id").and_then(EmojiId::decode)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            managed: req!(try!(remove(&mut map, "managed")).as_bool()),
            require_colons: req!(try!(remove(&mut map, "require_colons")).as_bool()),
            roles: try!(remove(&mut map, "roles").and_then(|v| decode_array(v, RoleId::decode))),
        })
    }
}

/// Version of emoji struct used only when Id and name are known.
#[derive(Clone, Debug)]
pub struct EmojiIdentifier {
    /// The Id of the emoji.
    pub id: EmojiId,
    /// The name of the emoji. It must be at least 2 characters long and can only contain alphanumeric characters and underscores.
    pub name: String,
}

impl EmojiIdentifier {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<EmojiIdentifier> {
        let mut map = try!(into_map(value));

        Ok(EmojiIdentifier {
            id: try!(remove(&mut map, "id").and_then(EmojiId::decode)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
        })
    }
}

/// Flags about who may or may not add the current user as a friend.
#[derive(Clone, Debug)]
pub struct FriendSourceFlags {
    /// 
    pub all: bool,
    /// 
    pub mutual_friends: bool,
    /// 
    pub mutual_guilds: bool,
}

impl FriendSourceFlags {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<FriendSourceFlags> {
        let mut map = try!(into_map(value));

        Ok(FriendSourceFlags {
            all: try!(opt(&mut map, "all", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            mutual_friends: try!(opt(&mut map, "mutual_friends", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            mutual_guilds: try!(opt(&mut map, "mutual_guilds", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
        })
    }
}

/// Represents a game that a a user is playing, or streaming in the case that a stream link is provided.
#[derive(Clone, Debug)]
pub struct Game {
    /// The type of game status.
    pub kind: GameType,
    /// The name of the game that's being played.
    pub name: String,
    /// Stream URL if the `kind` is `Streaming`.
    pub url: Option<String>,
}

/// A representation of the data retrieved from the gateway endpoint.
/// 
/// For the bot-specific gateway, refer to [`BotGateway`].
/// 
///   [`BotGateway`]: struct.BotGateway.html
#[derive(Clone, Debug)]
pub struct Gateway {
    /// The gateway to connect to.
    pub url: String,
}

impl Gateway {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Gateway> {
        let mut map = try!(into_map(value));

        Ok(Gateway {
            url: try!(remove(&mut map, "url").and_then(into_string)),
        })
    }
}

/// A group channel, potentially including other users, separate from a [`Guild`].
/// 
///   [`Guild`]: struct.Guild.html
#[derive(Clone, Debug)]
pub struct Group {
    /// The Id of the group channel.
    pub channel_id: ChannelId,
    /// The optional icon of the group channel.
    pub icon: Option<String>,
    /// The Id of the last message sent.
    pub last_message_id: Option<MessageId>,
    /// Timestamp of the latest pinned message.
    pub last_pin_timestamp: Option<String>,
    /// The name of the group channel.
    pub name: Option<String>,
    /// The Id of the group channel creator.
    pub owner_id: UserId,
    /// Group channel's members.
    pub recipients: HashMap<UserId, User>,
}

impl Group {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Group> {
        let mut map = try!(into_map(value));

        Ok(Group {
            channel_id: try!(remove(&mut map, "id").and_then(ChannelId::decode)),
            icon: try!(opt(&mut map, "icon", into_string)),
            last_message_id: try!(opt(&mut map, "last_message_id", MessageId::decode)),
            last_pin_timestamp: try!(opt(&mut map, "last_pin_timestamp", into_string)),
            name: try!(opt(&mut map, "name", into_string)),
            owner_id: try!(remove(&mut map, "owner_id").and_then(UserId::decode)),
            recipients: try!(remove(&mut map, "recipients").and_then(decode_users)),
        })
    }
}

/// Information about a Discord guild such as channels, emojis, etc.
#[derive(Clone, Debug)]
pub struct Guild {
    /// Id of a voice channel that's considered AFK.
    pub afk_channel_id: Option<ChannelId>,
    /// The amount of seconds a user can not show any activity in a voice channel before being moved to an AFK channel if one exists.
    pub afk_timeout: u64,
    /// All voice and text channels a guild has. This gives all of them regardless of permissions.
    pub channels: HashMap<ChannelId, GuildChannel>,
    /// Lets you know if notifications for all messages are enabled by default in the guild.
    pub default_message_notifications: u64,
    /// All custom emojis of a guild. Such are made using the API or Twitch integrations.
    pub emojis: HashMap<EmojiId, Emoji>,
    /// VIP guild features a guild has. Can be obtained through [Discord Partnership] website.
    ///
    ///  [Discord Partnership]: https://discordapp.com/partners
    pub features: Vec<Feature>,
    /// Optional guild icon that appears in sidebar.
    pub icon: Option<String>,
    /// Guild's Id which is also the Id of the default role and channel.
    pub id: GuildId,
    /// 
    pub joined_at: String,
    /// Set to true if guild has a lot of users.
    ///
    ///True indicates that offline guild members aren't initially sent.
    pub large: bool,
    /// The amount of members in guild.
    pub member_count: u64,
    /// Members of the guild. Members might not all be available on start-up if the `large` field is `true`.
    pub members: HashMap<UserId, Member>,
    /// Indicator if guild requires 2-factor authentication for roles with certain permissions.
    pub mfa_level: u64,
    /// The guild's name.
    pub name: String,
    /// Id of the guild's owner.
    pub owner_id: UserId,
    /// Presence statuses of members.
    pub presences: HashMap<UserId, Presence>,
    /// The region that the guild's voice servers are located in.
    pub region: String,
    /// All roles a guild has.
    pub roles: HashMap<RoleId, Role>,
    /// If [InviteSplash] feature is enabled, this can point to splash image URL displayed when someone opens invite URL.
    ///
    ///  [InviteSplash]: enum.Feature.html#variant.InviteSplash
    pub splash: Option<String>,
    /// Determines the verification level.
    pub verification_level: VerificationLevel,
    /// Lets you know what voice channels user have joined.
    pub voice_states: HashMap<UserId, VoiceState>,
}

/// Represents a guild's voice or text channel. Some methods are available only for voice channels and some are only available for text channels.
#[derive(Clone, Debug)]
pub struct GuildChannel {
    /// Channel's Id. Default channel shares the Id with the guild it is in.
    pub id: ChannelId,
    /// Bitrate of channel. Only available for voice channels.
    pub bitrate: Option<u64>,
    /// Id of the guild the channel is located in.
    pub guild_id: GuildId,
    /// Type of the channel.
    pub kind: ChannelType,
    /// The Id of last message sent. It lets client determine if channel has unread messages.
    pub last_message_id: Option<MessageId>,
    /// Timestamp of the latest pinned message.
    pub last_pin_timestamp: Option<String>,
    /// Channel name. Voice and text channels have different limitations for this.
    pub name: String,
    /// Permission overwrites for members and roles.
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// Position of a channel.
    pub position: i64,
    /// Text channel topic.
    pub topic: Option<String>,
    /// Max amount of members allowed in a voice channel.
    pub user_limit: Option<u64>,
}

/// Information relating to a guild's embed.
#[derive(Clone, Debug)]
pub struct GuildEmbed {
    /// The ID of the [`Channel`] to show the embed for.
    ///
    ///  [`Channel`]: enum.Channel.html
    pub channel_id: ChannelId,
    /// Whether the embed is enabled.
    pub enabled: bool,
}

impl GuildEmbed {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<GuildEmbed> {
        let mut map = try!(into_map(value));

        Ok(GuildEmbed {
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            enabled: req!(try!(remove(&mut map, "enabled")).as_bool()),
        })
    }
}

/// Basic information about a guild used for oauth.
#[derive(Clone, Debug)]
pub struct GuildInfo {
    /// Id of the Guild. Can be used to calculate the creation date.
    pub id: GuildId,
    /// The guild's icon.
    pub icon: Option<String>,
    /// The guild's name.
    pub name: String,
    /// True if you're the owner of the guild.
    pub owner: bool,
    /// Permissions that you have in the guild.
    pub permissions: Permissions,
}

impl GuildInfo {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<GuildInfo> {
        let mut map = try!(into_map(value));

        Ok(GuildInfo {
            id: try!(remove(&mut map, "id").and_then(GuildId::decode)),
            icon: try!(opt(&mut map, "icon", into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            owner: req!(try!(remove(&mut map, "owner")).as_bool()),
            permissions: try!(remove(&mut map, "permissions").and_then(Permissions::decode)),
        })
    }
}

/// Representation of the number of members that would be pruned by a guild prune operation.
#[derive(Clone, Debug)]
pub struct GuildPrune {
    /// 
    pub pruned: u64,
}

impl GuildPrune {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<GuildPrune> {
        let mut map = try!(into_map(value));

        Ok(GuildPrune {
            pruned: req!(try!(remove(&mut map, "pruned")).as_u64()),
        })
    }
}

/// An incident retrieved from the Discord status page.
/// 
/// This is not necessarily a representation of an ongoing incident.
#[derive(Clone, Debug)]
pub struct Incident {
    /// 
    pub created_at: String,
    /// 
    pub id: String,
    /// 
    pub impact: String,
    /// 
    pub incident_updates: Vec<IncidentUpdate>,
    /// 
    pub monitoring_at: Option<String>,
    /// 
    pub name: String,
    /// 
    pub page_id: String,
    /// 
    pub resolved_at: Option<String>,
    /// 
    pub short_link: String,
    /// 
    pub status: String,
    /// 
    pub updated_at: String,
}

impl Incident {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Incident> {
        let mut map = try!(into_map(value));

        Ok(Incident {
            created_at: try!(remove(&mut map, "created_at").and_then(into_string)),
            id: try!(remove(&mut map, "id").and_then(into_string)),
            impact: try!(remove(&mut map, "impact").and_then(into_string)),
            incident_updates: try!(remove(&mut map, "incident_updates").and_then(|v| decode_array(v, IncidentUpdate::decode))),
            monitoring_at: try!(opt(&mut map, "monitoring_at", into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            page_id: try!(remove(&mut map, "page_id").and_then(into_string)),
            resolved_at: try!(opt(&mut map, "resolved_at", into_string)),
            short_link: try!(remove(&mut map, "short_link").and_then(into_string)),
            status: try!(remove(&mut map, "status").and_then(into_string)),
            updated_at: try!(remove(&mut map, "updated_at").and_then(into_string)),
        })
    }
}

/// An update to an incident from the Discord status page.
/// 
/// This will typically state what new information has been discovered about an incident.
#[derive(Clone, Debug)]
pub struct IncidentUpdate {
    /// 
    pub affected_components: Vec<AffectedComponent>,
    /// 
    pub body: String,
    /// 
    pub created_at: String,
    /// 
    pub display_at: String,
    /// 
    pub id: String,
    /// 
    pub incident_id: String,
    /// 
    pub status: IncidentStatus,
    /// 
    pub updated_at: String,
}

impl IncidentUpdate {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<IncidentUpdate> {
        let mut map = try!(into_map(value));

        Ok(IncidentUpdate {
            affected_components: try!(remove(&mut map, "affected_components").and_then(|v| decode_array(v, AffectedComponent::decode))),
            body: try!(remove(&mut map, "body").and_then(into_string)),
            created_at: try!(remove(&mut map, "created_at").and_then(into_string)),
            display_at: try!(remove(&mut map, "display_at").and_then(into_string)),
            id: try!(remove(&mut map, "id").and_then(into_string)),
            incident_id: try!(remove(&mut map, "incident_id").and_then(into_string)),
            status: try!(remove(&mut map, "status").and_then(IncidentStatus::decode)),
            updated_at: try!(remove(&mut map, "updated_at").and_then(into_string)),
        })
    }
}

/// Holds various information about integrations.
#[derive(Clone, Debug)]
pub struct Integration {
    /// 
    pub id: IntegrationId,
    /// 
    pub account: IntegrationAccount,
    /// 
    pub enabled: bool,
    /// 
    pub expire_behavior: u64,
    /// 
    pub expire_grace_period: u64,
    /// 
    pub kind: String,
    /// 
    pub name: String,
    /// 
    pub role_id: RoleId,
    /// 
    pub synced_at: u64,
    /// 
    pub syncing: bool,
    /// 
    pub user: User,
}

impl Integration {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Integration> {
        let mut map = try!(into_map(value));

        Ok(Integration {
            id: try!(remove(&mut map, "id").and_then(IntegrationId::decode)),
            account: try!(remove(&mut map, "account").and_then(IntegrationAccount::decode)),
            enabled: req!(try!(remove(&mut map, "enabled")).as_bool()),
            expire_behavior: req!(try!(remove(&mut map, "expire_behavior")).as_u64()),
            expire_grace_period: req!(try!(remove(&mut map, "expire_grace_period")).as_u64()),
            kind: try!(remove(&mut map, "kind").and_then(into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            role_id: try!(remove(&mut map, "role_id").and_then(RoleId::decode)),
            synced_at: req!(try!(remove(&mut map, "synced_at")).as_u64()),
            syncing: req!(try!(remove(&mut map, "syncing")).as_bool()),
            user: try!(remove(&mut map, "user").and_then(User::decode)),
        })
    }
}

/// Integration Account Object
#[derive(Clone, Debug)]
pub struct IntegrationAccount {
    /// 
    pub id: String,
    /// 
    pub name: String,
}

impl IntegrationAccount {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<IntegrationAccount> {
        let mut map = try!(into_map(value));

        Ok(IntegrationAccount {
            id: try!(remove(&mut map, "id").and_then(into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
        })
    }
}

/// Information about an invite URL. Can't be accessed if the current user is banned.
#[derive(Clone, Debug)]
pub struct Invite {
    /// The unique code for the invite.
    pub code: String,
    /// A representation of the minimal amount of information needed about the [`GuildChannel`] being invited to.
    ///
    ///  [`GuildChannel`]: struct.GuildChannel.html
    pub channel: InviteChannel,
    /// A representation of the minimal amount of information needed about the [`Guild`] being invited to.
    ///
    ///  [`Guild`]: struct.Guild.html
    pub guild: InviteGuild,
}

impl Invite {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Invite> {
        let mut map = try!(into_map(value));

        Ok(Invite {
            code: try!(remove(&mut map, "code").and_then(into_string)),
            channel: try!(remove(&mut map, "channel").and_then(InviteChannel::decode)),
            guild: try!(remove(&mut map, "guild").and_then(InviteGuild::decode)),
        })
    }
}

/// Minimal information about the channel an invite points to.
#[derive(Clone, Debug)]
pub struct InviteChannel {
    /// 
    pub id: ChannelId,
    /// 
    pub name: String,
    /// 
    pub kind: ChannelType,
}

impl InviteChannel {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<InviteChannel> {
        let mut map = try!(into_map(value));

        Ok(InviteChannel {
            id: try!(remove(&mut map, "id").and_then(ChannelId::decode)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            kind: try!(remove(&mut map, "type").and_then(ChannelType::decode)),
        })
    }
}

/// A minimal amount of information about an invite's guild.
#[derive(Clone, Debug)]
pub struct InviteGuild {
    /// 
    pub id: GuildId,
    /// 
    pub icon: Option<String>,
    /// 
    pub name: String,
    /// 
    pub splash_hash: Option<String>,
}

impl InviteGuild {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<InviteGuild> {
        let mut map = try!(into_map(value));

        Ok(InviteGuild {
            id: try!(remove(&mut map, "id").and_then(GuildId::decode)),
            icon: try!(opt(&mut map, "icon", into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            splash_hash: try!(opt(&mut map, "splash_hash", into_string)),
        })
    }
}

/// A Discord status maintenance message. This can be either for active maintenances or for scheduled maintenances.
#[derive(Clone, Debug)]
pub struct Maintenance {
    /// 
    pub description: String,
    /// 
    pub id: String,
    /// 
    pub name: String,
    /// 
    pub start: String,
    /// 
    pub stop: String,
}

impl Maintenance {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Maintenance> {
        let mut map = try!(into_map(value));

        Ok(Maintenance {
            description: try!(remove(&mut map, "description").and_then(into_string)),
            id: try!(remove(&mut map, "id").and_then(into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            start: try!(remove(&mut map, "start").and_then(into_string)),
            stop: try!(remove(&mut map, "stop").and_then(into_string)),
        })
    }
}

/// Represents information about a member of a guild.
#[derive(Clone, Debug)]
pub struct Member {
    /// True if user isn't allowed to hear in voice channels.
    pub deaf: bool,
    /// Timestamp representing the date when the member joined.
    pub joined_at: String,
    /// True if user isn't allowed to talk in voice channels.
    pub mute: bool,
    /// Optional nickname. Can't be longer than 32 characters.
    pub nick: Option<String>,
    /// Ids of roles that were given to the member.
    pub roles: Vec<RoleId>,
    /// Attached User struct.
    pub user: User,
}

impl Member {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Member> {
        let mut map = try!(into_map(value));

        Ok(Member {
            deaf: req!(try!(remove(&mut map, "deaf")).as_bool()),
            joined_at: try!(remove(&mut map, "joined_at").and_then(into_string)),
            mute: req!(try!(remove(&mut map, "mute")).as_bool()),
            nick: try!(opt(&mut map, "nick", into_string)),
            roles: try!(remove(&mut map, "roles").and_then(|v| decode_array(v, RoleId::decode))),
            user: try!(remove(&mut map, "user").and_then(User::decode)),
        })
    }
}

/// A representation of a message sent over a guild's text channel, a group, or a private channel.
#[derive(Clone, Debug)]
pub struct Message {
    /// Message Id. Can be used to calculate the message creation date.
    pub id: MessageId,
    /// Array of attached files a message has.
    pub attachments: Vec<Attachment>,
    /// User that sent the message.
    pub author: User,
    /// Channel to which the message was sent.
    pub channel_id: ChannelId,
    /// Message's content.
    pub content: String,
    /// If the message was edited, this will show the last edit timestamp.
    pub edited_timestamp: Option<String>,
    /// Array of embeds a message has.
    pub embeds: Vec<Embed>,
    /// Whether the message is the "found" message in a search. Note that this is only relevant in the context of searches, and will otherwise always be `false`.
    pub hit: bool,
    /// Lets you differentiate system messages and regular messages.
    pub kind: MessageType,
    /// Shows you whether this message actually mentions everyone or not.
    pub mention_everyone: bool,
    /// Array of roles mentioned by the message.
    pub mention_roles: Vec<RoleId>,
    /// Array of users mentioned by the messages.
    pub mentions: Vec<User>,
    /// Non-repeating number used for ensuring message order.
    pub nonce: Option<String>,
    /// True if message is pinned.
    pub pinned: bool,
    /// Array of reactions performed on the message.
    pub reactions: Vec<MessageReaction>,
    /// Initial message creation timestamp calculated from Id.
    pub timestamp: String,
    /// True if message was sent with /tts command.
    pub tts: bool,
    /// An id of a webhook if message was sent using one.
    pub webhook_id: Option<WebhookId>,
}

impl Message {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Message> {
        let mut map = try!(into_map(value));

        Ok(Message {
            id: try!(remove(&mut map, "id").and_then(MessageId::decode)),
            attachments: try!(remove(&mut map, "attachments").and_then(|v| decode_array(v, Attachment::decode))),
            author: try!(remove(&mut map, "author").and_then(User::decode)),
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            content: try!(remove(&mut map, "content").and_then(into_string)),
            edited_timestamp: try!(opt(&mut map, "edited_timestamp", into_string)),
            embeds: try!(remove(&mut map, "embeds").and_then(|v| decode_array(v, Embed::decode))),
            hit: try!(opt(&mut map, "hit", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            kind: try!(remove(&mut map, "type").and_then(MessageType::decode)),
            mention_everyone: req!(try!(remove(&mut map, "mention_everyone")).as_bool()),
            mention_roles: try!(remove(&mut map, "mention_roles").and_then(|v| decode_array(v, RoleId::decode))),
            mentions: try!(remove(&mut map, "mentions").and_then(|v| decode_array(v, User::decode))),
            nonce: try!(opt(&mut map, "nonce", into_string)),
            pinned: req!(try!(remove(&mut map, "pinned")).as_bool()),
            reactions: try!(opt(&mut map, "reactions", |v| decode_array(v, MessageReaction::decode))).unwrap_or(Vec::default()),
            timestamp: try!(remove(&mut map, "timestamp").and_then(into_string)),
            tts: req!(try!(remove(&mut map, "tts")).as_bool()),
            webhook_id: try!(opt(&mut map, "webhook_id", WebhookId::decode)),
        })
    }
}

/// A representation of a [`Reaction`] to a [`Message`], where multiple of the same [reaction type] are stacked into one `MessageReaction`, with an associated [`count`].
/// 
/// 
///   [`Message`]: struct.Message.html
///   [`Reaction`]: struct.Reaction.html
///   [`count`]: #structfield.count
///   [reaction type]: enum.ReactionType.html
#[derive(Clone, Debug)]
pub struct MessageReaction {
    /// The amount of the type of reaction that have been sent for the associated message.
    pub count: u64,
    /// Whether the current user has sent the type of reaction.
    pub me: bool,
    /// The type of reaction.
    pub reaction_type: ReactionType,
}

impl MessageReaction {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<MessageReaction> {
        let mut map = try!(into_map(value));

        Ok(MessageReaction {
            count: req!(try!(remove(&mut map, "count")).as_u64()),
            me: req!(try!(remove(&mut map, "me")).as_bool()),
            reaction_type: try!(remove(&mut map, "emoji").and_then(ReactionType::decode)),
        })
    }
}

/// Partial information about a guild. This does not include information like member data.
#[derive(Clone, Debug)]
pub struct PartialGuild {
    /// 
    pub id: GuildId,
    /// 
    pub afk_channel_id: Option<ChannelId>,
    /// 
    pub afk_timeout: u64,
    /// 
    pub default_message_notifications: u64,
    /// 
    pub embed_channel_id: Option<ChannelId>,
    /// 
    pub embed_enabled: bool,
    /// 
    pub emojis: HashMap<EmojiId, Emoji>,
    /// 
    pub features: Vec<Feature>,
    /// 
    pub icon: Option<String>,
    /// 
    pub mfa_level: u64,
    /// 
    pub name: String,
    /// 
    pub owner_id: UserId,
    /// 
    pub region: String,
    /// 
    pub roles: HashMap<RoleId, Role>,
    /// 
    pub splash: Option<String>,
    /// 
    pub verification_level: VerificationLevel,
}

impl PartialGuild {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<PartialGuild> {
        let mut map = try!(into_map(value));

        Ok(PartialGuild {
            id: try!(remove(&mut map, "id").and_then(GuildId::decode)),
            afk_channel_id: try!(opt(&mut map, "afk_channel_id", ChannelId::decode)),
            afk_timeout: req!(try!(remove(&mut map, "afk_timeout")).as_u64()),
            default_message_notifications: req!(try!(remove(&mut map, "default_message_notifications")).as_u64()),
            embed_channel_id: try!(opt(&mut map, "embed_channel_id", ChannelId::decode)),
            embed_enabled: req!(try!(remove(&mut map, "embed_enabled")).as_bool()),
            emojis: try!(remove(&mut map, "emojis").and_then(decode_emojis)),
            features: try!(remove(&mut map, "features").and_then(|v| decode_array(v, Feature::decode_str))),
            icon: try!(opt(&mut map, "icon", into_string)),
            mfa_level: req!(try!(remove(&mut map, "mfa_level")).as_u64()),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            owner_id: try!(remove(&mut map, "owner_id").and_then(UserId::decode)),
            region: try!(remove(&mut map, "region").and_then(into_string)),
            roles: try!(remove(&mut map, "roles").and_then(decode_roles)),
            splash: try!(opt(&mut map, "splash", into_string)),
            verification_level: try!(remove(&mut map, "verification_level").and_then(VerificationLevel::decode)),
        })
    }
}

/// A channel-specific permission overwrite for a member or role.
#[derive(Clone, Debug)]
pub struct PermissionOverwrite {
    /// 
    pub allow: Permissions,
    /// 
    pub deny: Permissions,
    /// 
    pub kind: PermissionOverwriteType,
}

/// A set of settings each member of a guild has.
#[derive(Clone, Debug)]
pub struct Presence {
    /// A game's name that is displayed near user's name.
    pub game: Option<Game>,
    /// Date of last presence change.
    pub last_modified: Option<u64>,
    /// Optional nickname. Can't be longer than 32 characters.
    pub nick: Option<String>,
    /// Member's online status.
    pub status: OnlineStatus,
    /// Member's Id. Can be used to calculate account creation date.
    pub user_id: UserId,
    /// Attached User struct.
    pub user: Option<User>,
}

/// A Direct Message text channel with another user.
#[derive(Clone, Debug)]
pub struct PrivateChannel {
    /// Private channel's Id. Can be used to calculate the first message's creation date.
    pub id: ChannelId,
    /// The Id of last message sent. It lets client determine if the channel has unread messages.
    pub last_message_id: Option<MessageId>,
    /// Timestamp of the latest pinned message.
    pub last_pin_timestamp: Option<String>,
    /// Lets you differentiate between channel types.
    pub kind: ChannelType,
    /// User that receives messages in this channel.
    pub recipient: User,
}

/// An [`Emoji`] reaction to a [`Message`].
/// 
///   [`Emoji`]: struct.Emoji.html
///   [`Message`]: struct.Message.html
#[derive(Clone, Debug)]
pub struct Reaction {
    /// The [`Channel`] of the associated [`Message`].
    ///
    ///  [`Channel`]: enum.Channel.html
    ///  [`Message`]: struct.Message.html
    pub channel_id: ChannelId,
    /// The reactive emoji used.
    pub emoji: ReactionType,
    /// The [`Message`] that was reacted to.
    ///
    ///  [`Message`]: struct.Message.html
    pub message_id: MessageId,
    /// The Id of the [`User`] that sent the reaction.
    ///
    ///  [`User`]: struct.User.html
    pub user_id: UserId,
}

impl Reaction {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Reaction> {
        let mut map = try!(into_map(value));

        Ok(Reaction {
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            emoji: try!(remove(&mut map, "emoji").and_then(ReactionType::decode)),
            message_id: try!(remove(&mut map, "message_id").and_then(MessageId::decode)),
            user_id: try!(remove(&mut map, "user_id").and_then(UserId::decode)),
        })
    }
}

/// Summary of messages since last login.
#[derive(Clone, Debug)]
pub struct ReadState {
    /// The channel's Id.
    pub id: ChannelId,
    /// The Id of the latest message sent to the channel.
    pub last_message_id: Option<MessageId>,
    /// The timestmap of the latest pinned message in the channel.
    pub last_pin_timestamp: Option<String>,
    /// The amount of times you've been mentioned in the channel.
    pub mention_count: u64,
}

impl ReadState {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<ReadState> {
        let mut map = try!(into_map(value));

        Ok(ReadState {
            id: try!(remove(&mut map, "id").and_then(ChannelId::decode)),
            last_message_id: try!(opt(&mut map, "last_message_id", MessageId::decode)),
            last_pin_timestamp: try!(opt(&mut map, "last_pin_timestamp", into_string)),
            mention_count: try!(opt(&mut map, "mention_count", |v| Ok(req!(v.as_u64())))).unwrap_or(0),
        })
    }
}

/// An active group or private call. These are different from guild voice channels.
#[derive(Clone, Debug)]
pub struct Ready {
    /// 
    pub analytics_token: Option<String>,
    /// 
    pub experiments: Option<Vec<Vec<u64>>>,
    /// 
    pub friend_suggestion_count: Option<u64>,
    /// 
    pub guilds: Vec<PossibleGuild<Guild>>,
    /// The shard info for this session; the shard is used and the total number of shards.
    pub notes: HashMap<UserId, String>,
    /// 
    pub presences: HashMap<UserId, Presence>,
    /// 
    pub private_channels: HashMap<ChannelId, Channel>,
    /// 
    pub read_state: HashMap<ChannelId, ReadState>,
    /// 
    pub relationships: HashMap<UserId, Relationship>,
    /// 
    pub session_id: String,
    /// 
    pub shard: Option<[u64; 2]>,
    /// The trace of guilds involved in this connection.
    pub trace: Option<Vec<String>>,
    /// 
    pub tutorial: Option<Tutorial>,
    /// 
    pub user: CurrentUser,
    /// 
    pub user_guild_settings: Option<Vec<UserGuildSettings>>,
    /// 
    pub user_settings: Option<UserSettings>,
    /// 
    pub version: u64,
}

impl Ready {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Ready> {
        let mut map = try!(into_map(value));

        Ok(Ready {
            analytics_token: try!(opt(&mut map, "analytics_token", into_string)),
            experiments: try!(opt(&mut map, "experiments", decode_experiments)),
            friend_suggestion_count: remove(&mut map, "friend_suggestion_count").ok().and_then(|v| v.as_u64()),
            guilds: try!(remove(&mut map, "guilds").and_then(|v| decode_array(v, PossibleGuild::<Guild>::decode))),
            notes: try!(opt(&mut map, "notes", decode_notes)).unwrap_or(HashMap::default()),
            presences: try!(remove(&mut map, "presences").and_then(decode_presences)),
            private_channels: try!(remove(&mut map, "private_channels").and_then(decode_private_channels)),
            read_state: try!(opt(&mut map, "read_state", decode_read_states)).unwrap_or(HashMap::default()),
            relationships: try!(remove(&mut map, "relationships").and_then(decode_relationships)),
            session_id: try!(remove(&mut map, "session_id").and_then(into_string)),
            shard: try!(opt(&mut map, "shard", decode_shards)),
            trace: try!(opt(&mut map, "_trace", |v| decode_array(v, into_string))),
            tutorial: try!(opt(&mut map, "tutorial", Tutorial::decode)),
            user: try!(remove(&mut map, "user").and_then(CurrentUser::decode)),
            user_guild_settings: try!(opt(&mut map, "user_guild_settings", |v| decode_array(v, UserGuildSettings::decode))),
            user_settings: try!(remove(&mut map, "user_settings").and_then(UserSettings::decode)),
            version: req!(try!(remove(&mut map, "v")).as_u64()),
        })
    }
}

/// Information about a relationship that a user has with another user.
#[derive(Clone, Debug)]
pub struct Relationship {
    /// Id of the first relationship participant.
    pub id: UserId,
    /// Type of the relationship such as blocked, friends etc.
    pub kind: RelationshipType,
    /// User struct of the second relationship participant.
    pub user: User,
}

impl Relationship {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Relationship> {
        let mut map = try!(into_map(value));

        Ok(Relationship {
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            kind: try!(remove(&mut map, "type").and_then(RelationshipType::decode)),
            user: try!(remove(&mut map, "user").and_then(User::decode)),
        })
    }
}

/// Detailed information about an invite. This information can only be retrieved by anyone with the [Manage Guild] permission. Otherwise, a minimal amount of information can be retrieved via the [`Invite`] struct.
/// 
///   [`Invite`]: struct.Invite.html
///   [Manage Guild]: permissions/constant.MANAGE_GUILD.html
#[derive(Clone, Debug)]
pub struct RichInvite {
    /// A representation of the minimal amount of information needed about the [`GuildChannel`] being invited to.
    ///
    ///  [`GuildChannel`]: struct.GuildChannel.html
    pub channel: InviteChannel,
    /// The unique code for the invite.
    pub code: String,
    /// When the invite was created.
    pub created_at: String,
    /// A representation of the minimal amount of information needed about the [`Guild`] being invited to.
    ///
    ///  [`Guild`]: struct.Guild.html
    pub guild: InviteGuild,
    /// The user that created the invite.
    pub inviter: User,
    /// The maximum age of the invite in seconds, from when it was created.
    pub max_age: u64,
    /// The maximum number of times that an invite may be used before it expires.
    ///Note that this does not supercede the [`max_age`] value, if the value of [`temporary`] is `true`. If the value of `temporary` is `false`, then the invite _will_ self-expire after the given number of max uses.
    ///If the value is `0`, then the invite is permanent.
    ///
    ///  [`max_age`]: #structfield.max_age
    ///  [`temporary`]: #structfield.temporary
    pub max_uses: u64,
    /// Whether the invite self-expires after a certain amount of time or uses.
    pub temporary: bool,
    /// The maximum amount of times that an invite may be used before it self-expires.
    pub uses: u64,
}

impl RichInvite {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<RichInvite> {
        let mut map = try!(into_map(value));

        Ok(RichInvite {
            channel: try!(remove(&mut map, "channel").and_then(InviteChannel::decode)),
            code: try!(remove(&mut map, "code").and_then(into_string)),
            created_at: try!(remove(&mut map, "created_at").and_then(into_string)),
            guild: try!(remove(&mut map, "guild").and_then(InviteGuild::decode)),
            inviter: try!(remove(&mut map, "inviter").and_then(User::decode)),
            max_age: req!(try!(remove(&mut map, "max_age")).as_u64()),
            max_uses: req!(try!(remove(&mut map, "max_uses")).as_u64()),
            temporary: req!(try!(remove(&mut map, "temporary")).as_bool()),
            uses: req!(try!(remove(&mut map, "uses")).as_u64()),
        })
    }
}

/// Information about a role within a guild. A role represents a set of permissions, and can be attached to one or multiple users. A role has various miscellaneous configurations, such as being assigned a colour. Roles are unique per guild and do not cross over to other guilds in any way, and can have channel-specific permission overrides in addition to guild-level permissions.
#[derive(Clone, Debug)]
pub struct Role {
    /// The Id of the role. Can be used to calculate its creation date.
    pub id: RoleId,
    /// The colour of the role in 0xRRGGBB format. This is an ergonomic representation of the inner value.
    pub colour: Colour,
    /// Whether the role is pinned above lesser roles, causing members in the role to be seen above others.
    pub hoist: bool,
    /// Whether the role is managed by an integration service.
    pub managed: bool,
    /// Whether the role can be mentioned, similar to mentioning a specific member or @everyone. Only members of the role will be notified if a role is mentioned with this set to true.
    pub mentionable: bool,
    /// The name of the role.
    pub name: String,
    /// A set of permissions that the role has been assigned. See the [`permissions`] module for more information.
    ///
    ///  [`permissions`]: permissions/index.html
    pub permissions: Permissions,
    /// The role's position in the position list. Roles above another are considered higher in the role hierarchy in most situations.
    ///The @everyone role is usually either position `-1` or `0`.
    pub position: i64,
}

impl Role {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Role> {
        let mut map = try!(into_map(value));

        Ok(Role {
            id: try!(remove(&mut map, "id").and_then(RoleId::decode)),
            colour: try!(remove(&mut map, "color").and_then(Colour::decode)),
            hoist: req!(try!(remove(&mut map, "hoist")).as_bool()),
            managed: req!(try!(remove(&mut map, "managed")).as_bool()),
            mentionable: req!(try!(remove(&mut map, "mentionable")).as_bool()),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            permissions: try!(remove(&mut map, "permissions").and_then(Permissions::decode)),
            position: req!(try!(remove(&mut map, "position")).as_i64()),
        })
    }
}

/// The results of a search, including the total results and a vector of messages.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// An array of messages returned from the search. Note that this is an array of an array of messages. Each "set" of messages is the "found" message, as well as surrounding messages for context.
    pub results: Vec<Vec<Message>>,
    /// The number of [message][`Message`]s directly related to the search. This does not count messages returned for context.
    ///
    ///  [`Message`]: struct.Message.html
    pub total: u64,
}

impl SearchResult {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<SearchResult> {
        let mut map = try!(into_map(value));

        Ok(SearchResult {
            results: try!(remove(&mut map, "messages").and_then(decode_search_results)),
            total: req!(try!(remove(&mut map, "total_results")).as_u64()),
        })
    }
}

/// A reason that a [`User`] was suggested to be added as a friend.
/// 
///   [`User`]: struct.User.html
#[derive(Clone, Debug)]
pub struct SuggestionReason {
    /// The name of the user on the platform.
    pub name: String,
    /// The platform that you and the user share.
    pub platform: ConnectionType,
    /// The type of reason.
    pub kind: u64,
}

impl SuggestionReason {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<SuggestionReason> {
        let mut map = try!(into_map(value));

        Ok(SuggestionReason {
            name: try!(remove(&mut map, "name").and_then(into_string)),
            platform: try!(remove(&mut map, "platform").and_then(ConnectionType::decode_str)),
            kind: req!(try!(remove(&mut map, "kind")).as_u64()),
        })
    }
}

/// The current user's progress through the Discord tutorial.
#[derive(Clone, Debug)]
pub struct Tutorial {
    /// 
    pub indicators_confirmed: Vec<String>,
    /// 
    pub indicators_suppressed: bool,
}

impl Tutorial {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Tutorial> {
        let mut map = try!(into_map(value));

        Ok(Tutorial {
            indicators_confirmed: try!(remove(&mut map, "indicators_confirmed").and_then(|v| decode_array(v, into_string))),
            indicators_suppressed: req!(try!(remove(&mut map, "indicators_suppressed")).as_bool()),
        })
    }
}

/// Information about a user.
#[derive(Clone, Debug)]
pub struct User {
    /// The unique Id of the user. Can be used to calculate the account's creation date.
    pub id: UserId,
    /// Optional avatar hash.
    pub avatar: Option<String>,
    /// Lets you know if the account is a bot or not.
    pub bot: bool,
    /// The account's discriminator. The name + discriminator pair is always unique.
    pub discriminator: String,
    /// The account's username. Changing username will trigger a discriminator change if the pair is not unique.
    pub name: String,
}

impl User {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<User> {
        let mut map = try!(into_map(value));

        Ok(User {
            id: try!(remove(&mut map, "id").and_then(UserId::decode)),
            avatar: try!(opt(&mut map, "avatar", into_string)),
            bot: try!(opt(&mut map, "bot", |v| Ok(req!(v.as_bool())))).unwrap_or(false),
            discriminator: try!(remove(&mut map, "discriminator").and_then(into_string)),
            name: try!(remove(&mut map, "username").and_then(into_string)),
        })
    }
}

/// A user's connection.
/// 
/// **Note**: This is not in any way related to a WebSocket connection.
#[derive(Clone, Debug)]
pub struct UserConnection {
    /// The user's ID through the connection.
    pub id: String,
    /// Whether the user automatically syncs friends through the connection.
    pub friend_sync: bool,
    /// The relevant integrations.
    pub integrations: Vec<Integration>,
    /// The type of connection set.
    pub kind: ConnectionType,
    /// The user's name through the connection.
    pub name: String,
    /// Whether the connection has been revoked.
    pub revoked: bool,
    /// The visibility level.
    pub visibility: u64,
}

impl UserConnection {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<UserConnection> {
        let mut map = try!(into_map(value));

        Ok(UserConnection {
            id: try!(remove(&mut map, "id").and_then(into_string)),
            friend_sync: req!(try!(remove(&mut map, "friend_sync")).as_bool()),
            integrations: try!(remove(&mut map, "integrations").and_then(|v| decode_array(v, Integration::decode))),
            kind: try!(remove(&mut map, "type").and_then(ConnectionType::decode_str)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            revoked: req!(try!(remove(&mut map, "revoked")).as_bool()),
            visibility: req!(try!(remove(&mut map, "visibility")).as_u64()),
        })
    }
}

/// Settings about a guild in regards to notification configuration
#[derive(Clone, Debug)]
pub struct UserGuildSettings {
    /// 
    pub channel_overrides: Vec<ChannelOverride>,
    /// 
    pub guild_id: Option<GuildId>,
    /// 
    pub message_notifications: NotificationLevel,
    /// 
    pub mobile_push: bool,
    /// 
    pub muted: bool,
    /// 
    pub suppress_everyone: bool,
}

impl UserGuildSettings {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<UserGuildSettings> {
        let mut map = try!(into_map(value));

        Ok(UserGuildSettings {
            channel_overrides: try!(remove(&mut map, "channel_overrides").and_then(|v| decode_array(v, ChannelOverride::decode))),
            guild_id: try!(opt(&mut map, "guild_id", GuildId::decode)),
            message_notifications: try!(remove(&mut map, "message_notifications").and_then(NotificationLevel::decode)),
            mobile_push: req!(try!(remove(&mut map, "mobile_push")).as_bool()),
            muted: req!(try!(remove(&mut map, "muted")).as_bool()),
            suppress_everyone: req!(try!(remove(&mut map, "suppress_everyone")).as_bool()),
        })
    }
}

/// User settings usually used to influence client behavior.
#[derive(Clone, Debug)]
pub struct UserSettings {
    /// 
    pub convert_emoticons: bool,
    /// 
    pub enable_tts_command: bool,
    /// 
    pub friend_source_flags: FriendSourceFlags,
    /// 
    pub inline_attachment_media: bool,
    /// 
    pub inline_embed_media: bool,
    /// 
    pub locale: String,
    /// 
    pub message_display_compact: bool,
    /// 
    pub render_embeds: bool,
    /// 
    pub restricted_guilds: Vec<GuildId>,
    /// 
    pub show_current_game: bool,
    /// 
    pub status: OnlineStatus,
    /// 
    pub theme: String,
}

/// Information about an available voice region.
#[derive(Clone, Debug)]
pub struct VoiceRegion {
    /// Whether it is a custom voice region, which is used for events.
    pub custom: bool,
    /// Whether it is a deprecated voice region, which you should avoid using.
    pub deprecated: bool,
    /// The internal id of the voice region.
    pub id: String,
    /// A recognizable name of the location of the voice region.
    pub name: String,
    /// Whether the voice region is optimal for use for the current user.
    pub optimal: bool,
    /// An example hostname for the region.
    pub sample_hostname: String,
    /// An example port for the region.
    pub sample_port: u64,
    /// Whether the voice regions is only for VIPs.
    pub vip: bool,
}

impl VoiceRegion {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<VoiceRegion> {
        let mut map = try!(into_map(value));

        Ok(VoiceRegion {
            custom: req!(try!(remove(&mut map, "custom")).as_bool()),
            deprecated: req!(try!(remove(&mut map, "deprecated")).as_bool()),
            id: try!(remove(&mut map, "id").and_then(into_string)),
            name: try!(remove(&mut map, "name").and_then(into_string)),
            optimal: req!(try!(remove(&mut map, "optimal")).as_bool()),
            sample_hostname: try!(remove(&mut map, "sample_hostname").and_then(into_string)),
            sample_port: req!(try!(remove(&mut map, "sample_port")).as_u64()),
            vip: req!(try!(remove(&mut map, "vip")).as_bool()),
        })
    }
}

/// A member's state within a voice channel.
#[derive(Clone, Debug)]
pub struct VoiceState {
    /// 
    pub channel_id: Option<ChannelId>,
    /// 
    pub deaf: bool,
    /// 
    pub mute: bool,
    /// 
    pub self_deaf: bool,
    /// 
    pub self_mute: bool,
    /// 
    pub session_id: String,
    /// 
    pub suppress: bool,
    /// 
    pub token: Option<String>,
    /// 
    pub user_id: UserId,
}

impl VoiceState {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<VoiceState> {
        let mut map = try!(into_map(value));

        Ok(VoiceState {
            channel_id: try!(opt(&mut map, "channel_id", ChannelId::decode)),
            deaf: req!(try!(remove(&mut map, "deaf")).as_bool()),
            mute: req!(try!(remove(&mut map, "mute")).as_bool()),
            self_deaf: req!(try!(remove(&mut map, "self_deaf")).as_bool()),
            self_mute: req!(try!(remove(&mut map, "self_mute")).as_bool()),
            session_id: try!(remove(&mut map, "session_id").and_then(into_string)),
            suppress: req!(try!(remove(&mut map, "suppress")).as_bool()),
            token: try!(opt(&mut map, "token", into_string)),
            user_id: try!(remove(&mut map, "user_id").and_then(UserId::decode)),
        })
    }
}

/// A representation of a webhook, which is a low-effort way to post messages to channels. They do not necessarily require a bot user or authentication to use.
#[derive(Clone, Debug)]
pub struct Webhook {
    /// The Id of the webhook.
    pub id: WebhookId,
    /// The default avatar of the webhook. Note that this can be modified with a payload.
    pub avatar: Option<String>,
    /// The Id of the [channel][`GuildChannel`] that owns the webhook.
    ///
    ///  [`GuildChannel`]: struct.GuildChannel.html
    pub channel_id: ChannelId,
    /// The Id of the [`Guild`] that owns the webhook.
    ///
    ///  [`Guild`]: struct.Guild.html
    pub guild_id: Option<GuildId>,
    /// The default name of the webhook. Note that this can be modified in a payload via [`ExecuteWebhook::username`].
    ///
    ///  [`ExecuteWebhook::username`]: ../utils/builder/struct.ExecuteWebhook.html#method.username
    pub name: Option<String>,
    /// The webhook's secure token.
    pub token: String,
    /// The user that created the webhook.
    ///
    ///**Note**: This is not returned when getting a webhook by its token.
    pub user: Option<User>,
}

impl Webhook {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Webhook> {
        let mut map = try!(into_map(value));

        Ok(Webhook {
            id: try!(remove(&mut map, "id").and_then(WebhookId::decode)),
            avatar: try!(opt(&mut map, "avatar", into_string)),
            channel_id: try!(remove(&mut map, "channel_id").and_then(ChannelId::decode)),
            guild_id: try!(opt(&mut map, "guild_id", GuildId::decode)),
            name: try!(opt(&mut map, "name", into_string)),
            token: try!(remove(&mut map, "token").and_then(into_string)),
            user: try!(opt(&mut map, "user", User::decode)),
        })
    }
}