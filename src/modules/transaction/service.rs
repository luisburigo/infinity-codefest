// use crate::database::redis;
// use crate::models::user::User;
// 
// pub async fn create_transaction(payload: User) {
//     let user_id = payload.id.to_string();
//     let serialized_data = match serde_json::to_string(&payload) {
//         Ok(data) => data,
//         Err(e) => {
//             eprintln!("Error serializing user data: {:?}", e);
//             return;
//         }
//     };
// 
//     match db.set_key_value(&user_id, &serialized_data).await {
//         Ok(data) => {
//             println!("Change the event here type i guess... ?: {:?}", data)
//         }
//         Err(e) => {
//             eprintln!("Error while creating an user: {:?}", e);
//         }
//     };
// }
