#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "/v2";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AddPetResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid input
    InvalidInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeletePetResponse {
    /// Invalid pet value
    InvalidPetValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FindPetsByStatusResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid status value
    InvalidStatusValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FindPetsByTagsResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid tag value
    InvalidTagValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPetByIdResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid ID supplied
    InvalidIDSupplied,
    /// Pet not found
    PetNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePetResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid ID supplied
    InvalidIDSupplied,
    /// Pet not found
    PetNotFound,
    /// Validation exception
    ValidationException,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdatePetWithFormResponse {
    /// Invalid input
    InvalidInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UploadFileResponse {
    /// successful operation
    SuccessfulOperation(models::ApiResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteOrderResponse {
    /// Invalid ID supplied
    InvalidIDSupplied,
    /// Order not found
    OrderNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetInventoryResponse {
    /// successful operation
    SuccessfulOperation(std::collections::HashMap<String, i32>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetOrderByIdResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid ID supplied
    InvalidIDSupplied,
    /// Order not found
    OrderNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlaceOrderResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid Order
    InvalidOrder,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateUserResponse {
    /// successful operation
    SuccessfulOperation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateUsersWithArrayInputResponse {
    /// successful operation
    SuccessfulOperation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateUsersWithListInputResponse {
    /// successful operation
    SuccessfulOperation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteUserResponse {
    /// Invalid username supplied
    InvalidUsernameSupplied,
    /// User not found
    UserNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUserByNameResponse {
    /// successful operation
    SuccessfulOperation(String),
    /// Invalid username supplied
    InvalidUsernameSupplied,
    /// User not found
    UserNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LoginUserResponse {
    /// successful operation
    SuccessfulOperation {
        body: String,
        set_cookie: Option<String>,
        x_rate_limit: Option<i32>,
        x_expires_after: Option<chrono::DateTime<chrono::Utc>>,
    },
    /// Invalid username/password supplied
    InvalidUsername,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogoutUserResponse {
    /// successful operation
    SuccessfulOperation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateUserResponse {
    /// Invalid user supplied
    InvalidUserSupplied,
    /// User not found
    UserNotFound,
}

/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {
    /// Add a new pet to the store.
    ///
    /// AddPet - POST /v2/pet
    async fn add_pet(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Pet,
    ) -> Result<AddPetResponse, String>;

    /// Deletes a pet.
    ///
    /// DeletePet - DELETE /v2/pet/{petId}
    async fn delete_pet(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::DeletePetHeaderParams,
        path_params: models::DeletePetPathParams,
    ) -> Result<DeletePetResponse, String>;

    /// Finds Pets by status.
    ///
    /// FindPetsByStatus - GET /v2/pet/findByStatus
    async fn find_pets_by_status(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::FindPetsByStatusQueryParams,
    ) -> Result<FindPetsByStatusResponse, String>;

    /// Finds Pets by tags.
    ///
    /// FindPetsByTags - GET /v2/pet/findByTags
    async fn find_pets_by_tags(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::FindPetsByTagsQueryParams,
    ) -> Result<FindPetsByTagsResponse, String>;

    /// Find pet by ID.
    ///
    /// GetPetById - GET /v2/pet/{petId}
    async fn get_pet_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetPetByIdPathParams,
    ) -> Result<GetPetByIdResponse, String>;

    /// Update an existing pet.
    ///
    /// UpdatePet - PUT /v2/pet
    async fn update_pet(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Pet,
    ) -> Result<UpdatePetResponse, String>;

    /// Updates a pet in the store with form data.
    ///
    /// UpdatePetWithForm - POST /v2/pet/{petId}
    async fn update_pet_with_form(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdatePetWithFormPathParams,
    ) -> Result<UpdatePetWithFormResponse, String>;

    /// uploads an image.
    ///
    /// UploadFile - POST /v2/pet/{petId}/uploadImage
    async fn upload_file(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UploadFilePathParams,
        body: Multipart,
    ) -> Result<UploadFileResponse, String>;

    /// Delete purchase order by ID.
    ///
    /// DeleteOrder - DELETE /v2/store/order/{orderId}
    async fn delete_order(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteOrderPathParams,
    ) -> Result<DeleteOrderResponse, String>;

    /// Returns pet inventories by status.
    ///
    /// GetInventory - GET /v2/store/inventory
    async fn get_inventory(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<GetInventoryResponse, String>;

    /// Find purchase order by ID.
    ///
    /// GetOrderById - GET /v2/store/order/{orderId}
    async fn get_order_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetOrderByIdPathParams,
    ) -> Result<GetOrderByIdResponse, String>;

    /// Place an order for a pet.
    ///
    /// PlaceOrder - POST /v2/store/order
    async fn place_order(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Order,
    ) -> Result<PlaceOrderResponse, String>;

    /// Create user.
    ///
    /// CreateUser - POST /v2/user
    async fn create_user(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::User,
    ) -> Result<CreateUserResponse, String>;

    /// Creates list of users with given input array.
    ///
    /// CreateUsersWithArrayInput - POST /v2/user/createWithArray
    async fn create_users_with_array_input(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Vec<models::User>,
    ) -> Result<CreateUsersWithArrayInputResponse, String>;

    /// Creates list of users with given input array.
    ///
    /// CreateUsersWithListInput - POST /v2/user/createWithList
    async fn create_users_with_list_input(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Vec<models::User>,
    ) -> Result<CreateUsersWithListInputResponse, String>;

    /// Delete user.
    ///
    /// DeleteUser - DELETE /v2/user/{username}
    async fn delete_user(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteUserPathParams,
    ) -> Result<DeleteUserResponse, String>;

    /// Get user by user name.
    ///
    /// GetUserByName - GET /v2/user/{username}
    async fn get_user_by_name(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetUserByNamePathParams,
    ) -> Result<GetUserByNameResponse, String>;

    /// Logs user into the system.
    ///
    /// LoginUser - GET /v2/user/login
    async fn login_user(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::LoginUserQueryParams,
    ) -> Result<LoginUserResponse, String>;

    /// Logs out current logged in user session.
    ///
    /// LogoutUser - GET /v2/user/logout
    async fn logout_user(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<LogoutUserResponse, String>;

    /// Updated user.
    ///
    /// UpdateUser - PUT /v2/user/{username}
    async fn update_user(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateUserPathParams,
        body: models::User,
    ) -> Result<UpdateUserResponse, String>;
}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
