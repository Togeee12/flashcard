use crate::models;

// --- request type
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub enum AuthRequestType {
    #[serde(rename = "authenticate")]
    Authenticate,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "logout")]
    LogOut,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequestContent {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    #[serde(rename = "type")]
    pub request_type: AuthRequestType,
    pub content: Option<AuthRequestContent>,
}

// ---

#[derive(Debug, Clone, Deserialize)]
pub enum UsersRequestType {
    #[serde(rename = "get_my_profile")]
    GetMyProfile,
    #[serde(rename = "get_user")]
    GetUser,
    #[serde(rename = "create_user")]
    CreateUser,
    #[serde(rename = "update_user")]
    UpdateUser,
    #[serde(rename = "delete_user")]
    DeleteUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRequestContent {
    pub unique_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub date_of_registration: Option<u64>,
    pub country: Option<String>,
}

impl UserRequestContent {
    pub fn empty() -> Self {
        UserRequestContent { 
            unique_id: None,
            email: None,
            username: None,
            password: None,
            date_of_registration: None,
            country: None
        }
    }
}

pub trait ConvertUserFromOptional {
    fn try_from_optional(optional: &UserRequestContent) -> Result<Self, &'static str>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsersRequest {
    #[serde(rename = "type")]
    pub request_type: UsersRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<UserRequestContent>,
}

// ---

#[derive(Debug, Clone, Deserialize)]
pub enum CardsRequestType {
    #[serde(rename = "get_stacks_by_owner_id")]
    GetStacksByOwnerId,
    #[serde(rename = "get_stack_by_id")]
    GetStackById,
    #[serde(rename = "get_cards_by_stack_id")]
    GetCardsByStackId,
    #[serde(rename = "get_card_by_id")]
    GetCardById,
    #[serde(rename = "create_stack")]
    CreateStack,
    #[serde(rename = "create_card")]
    CreateCard,
    #[serde(rename = "update_stack")]
    UpdateStack,
    #[serde(rename = "update_card")]
    UpdateCard,
    #[serde(rename = "delete_stack")]
    DeleteStack,
    #[serde(rename = "delete_card")]
    DeleteCard,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CardsRequestContent {
    pub unique_id: Option<String>,
    pub stack_id: Option<String>,
    pub name: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<bool>,
    pub frontside: Option<String>,
    pub backside: Option<String>,
}

impl CardsRequestContent {
    pub fn empty() -> Self {
        CardsRequestContent { 
            unique_id: None,
            stack_id: None,
            name: None,
            tags: None,
            visibility: None,
            frontside: None,
            backside: None,
        }
    }
}

pub trait ConvertCardsFromOptional {
    fn try_from_optional(optional: &CardsRequestContent) -> Result<Self, &'static str>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Deserialize)]
pub struct CardsRequest {
    #[serde(rename = "type")]
    pub request_type: CardsRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<CardsRequestContent>,
}

// --- Response
use serde_derive::Serialize;


#[derive(Debug, Clone, Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "err")]
    Err,
}

#[derive(Debug, Clone, Serialize)]
pub enum ResponseError {
    #[allow(unused)]
    Debug,
    CouldntAuthenticate,
    EmailOrUSernameUsed,
    ParsingRequestContent,
    InvalidData,
    InvalidEmailOrPw,
    Unauthorized,
    InternalError,
}

impl ResponseError {
    pub fn parse(&self) -> ResponseErrorValues {
        match self {
            ResponseError::Debug =>ResponseErrorValues{c: -1, m: "Debug".to_owned()},
            ResponseError::CouldntAuthenticate =>ResponseErrorValues{c: 300, m: "Couldn't authenticate rquest".to_owned()},
            ResponseError::EmailOrUSernameUsed =>ResponseErrorValues{c: 310, m: "Email or username already in use".to_owned()},
            ResponseError::ParsingRequestContent =>ResponseErrorValues{c: 400, m: "Error parsing request conent".to_owned()},
            ResponseError::InvalidData =>ResponseErrorValues{c: 410, m: "Invalid content".to_owned()},
            ResponseError::InvalidEmailOrPw =>ResponseErrorValues{c: 411, m: "Invalid email or password".to_owned()},
            ResponseError::Unauthorized =>ResponseErrorValues{c: 430, m: "Unauthorized".to_owned()},
            ResponseError::InternalError =>ResponseErrorValues{c: 500, m: "Internal error".to_owned()},
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseErrorValues {
    #[serde(rename = "code")]
    c: i32,
    #[serde(rename = "message")]
    m: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserData {
    pub unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub username: String,
    pub date_of_registration: i64,
    pub country: String,
}

impl UserData {
    pub fn from(user: models::User, authorized: bool) -> Self {
        UserData {
            unique_id: user.unique_id,
            email: if authorized { Some(user.email) } else { None },
            username: user.username,
            date_of_registration: user.date_of_registration,
            country: user.country,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StackData {
    pub unique_id: String,
    pub name: String,
    pub cards_count: i32,
    pub tags: String,
    pub visibility: bool,
}

impl Into<StackData> for models::StackFull {
    fn into(self) -> StackData {
        StackData {
            unique_id: self.unique_id,
            name: self.name,
            cards_count: self.cards_count,
            tags: self.tags,
            visibility: self.visibility,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CardData {
    pub unique_id: String,
    pub frontside: String,
    pub backside: String,
}

impl Into<CardData> for models::Card {
    fn into(self) -> CardData {
        CardData {
            unique_id: self.unique_id,
            frontside: self.frontside,
            backside: self.backside,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ResponseErrorValues>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<UserData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stacks: Option<Vec<StackData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cards: Option<Vec<CardData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_id: Option<String>,
}

impl ResponseContent {
    pub fn new() -> Self {
        ResponseContent { errors: None, user: None, stacks: None, cards: None, authenticated: None, unique_id: None }
    }

    // pub fn push_err(&mut self, error: ResponseErrorValues) {
    //     match &mut self.errors {
    //         Some(value) => value.push(error),
    //         None => self.errors = Some(vec![error])
    //     }
    // }

    pub fn set_user(&mut self, user: UserData) {
        self.user = Some(user);
    }

    pub fn set_stacks(&mut self, stacks: Vec<StackData>) {
        self.stacks = Some(stacks);
    }

    pub fn set_cards(&mut self, cards: Vec<CardData>) {
        self.cards = Some(cards);
    }

    pub fn set_unique_id(&mut self, unique_id: &str) {
        self.unique_id = Some(unique_id.to_owned());
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ResponseContent>,
}

impl Response {
    pub fn new() -> Self {
        Response { 
            status: ResponseStatus::Ok, 
            content: Some( ResponseContent::new() ),
        }
    }

    pub fn empty_ok() -> Self {
        Response { 
            status: ResponseStatus::Ok, 
            content: None,
        }
    }

    pub fn new_err( errors: Vec<ResponseErrorValues> ) -> Self {
        Response { 
            status: ResponseStatus::Err, 
            content: Some( ResponseContent {
                errors: Some(errors),
                user: None,
                stacks: None,
                cards: None,
                authenticated: None,
                unique_id: None,
            }),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    // pub fn push_err(&mut self, error: ResponseErrorValues) {
    //     match &mut self.content {
    //         Some(content) => content.push_err(error),
    //         None => {
    //             let mut content = ResponseContent::new();
    //             content.push_err(error);
    //             self.content = Some(content);
    //         }
    //     }
    // }

    pub fn set_user(&mut self, user: UserData) -> &mut Self {
        match &mut self.content {
            Some(content) => content.set_user(user),
            None => {
                let mut content = ResponseContent::new();
                content.set_user(user);
                self.content = Some(content);
            }
        }
        self
    }

    pub fn set_stacks(&mut self, stacks: Vec<StackData>) -> &mut Self {
        match &mut self.content {
            Some(content) => content.set_stacks(stacks),
            None => {
                let mut content = ResponseContent::new();
                content.set_stacks(stacks);
                self.content = Some(content);
            }
        }
        self
    }

    pub fn set_cards(&mut self, cards: Vec<CardData>) -> &mut Self {
        match &mut self.content {
            Some(content) => content.set_cards(cards),
            None => {
                let mut content = ResponseContent::new();
                content.set_cards(cards);
                self.content = Some(content);
            }
        }
        self
    }

    pub fn set_unique_id(&mut self, unique_id: &str) -> &mut Self {
        match &mut self.content {
            Some(content) => content.set_unique_id(unique_id),
            None => {
                let mut content = ResponseContent::new();
                content.set_unique_id(unique_id);
                self.content = Some(content);
            }
        }
        self
    }
}