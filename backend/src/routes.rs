use actix_web::HttpResponse;

pub async fn http_404() -> HttpResponse {
    HttpResponse::Ok().body("404 Not found")
}


/*

Cheat sheet:


/auth: 
curl -i -X POST -H "Content-Type: application/json" -d '{"type": "authenticate", "content": {"email": "john.smith@hotmail.com", "password": "Dupa123!"}}' localhost:8000/api/v1/auth
curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTM2NjQzfQ.SKaxpyk4H03mVF6LCgD5JBmPIrsIdXCPA9gbEnYRf0c;" -d '{"type": "check"}' localhost:8000/api/v1/auth
curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTM2NjQzfQ.SKaxpyk4H03mVF6LCgD5JBmPIrsIdXCPA9gbEnYRf0c;" -d '{"type": "logout"}' localhost:8000/api/v1/auth


/users:
- get_my_profile:
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk4OTk1NzYyfQ.0Jci9JynctIhvBl11kEjfL4fHYnwFQFOuk4JE0DJ4dk;" -d '{"type": "get_my_profile"}' localhost:8000/api/v1/users
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjX29LN0k0QVk1IiwiZXhwIjoxNjk4NTQzMjEwfQ.sBx78txPa6nmUt_qWbzn4ZEQFK-iYuur4nLcxIqHU2E;" -d '{"type": "get_my_profile", "content": {}}' localhost:8000/api/v1/users

- get_user:
  - curl -i -X POST -H "Content-Type: application/json" -d '{"type": "get_user", "content": {"unique_id": "H8ZIe_honK"}}' localhost:8000/api/v1/users
  - curl -i -X POST -H "Content-Type: application/json" -d '{"type": "get_user", "content": {"username": "FlashCardEnjoyer69"}}' localhost:8000/api/v1/users

- create_user:
  - curl -i -X POST -H "Content-Type: application/json" -d '{"type": "create_user", "content": {"email": "john.smith@hotmail.com","username": "FlashCardEnjoyer69", "password": "Dupa123!", "country": "GBR"}}' localhost:8000/api/v1/users


/cards:
- get_stacks_by_owner_id:
  - curl -i -X POST -H "Content-Type: application/json" -d '{"type": "get_stacks_by_owner_id", "content": {"unique_id": "ukqk4u9oTn"}}' localhost:8000/api/v1/cards
- get_stack_by_id:
  - curl -i -X POST -H "Content-Type: application/json" -d '{"type": "get_stack_by_id", "content": {"unique_id": "8pX4VABFaw"}}' localhost:8000/api/v1/cards
- create_stack:
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTYxNTUxfQ.wz1kbm63J2Pzx61nXLn7hhy6FvLqzc45sN5W6r4mu98;" -d '{"type": "create_stack", "content": {"name": "my new stack", "tags": "my first stack, favourites", "visibility": true}}' localhost:8000/api/v1/cards
- create_card
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTYxNTUxfQ.wz1kbm63J2Pzx61nXLn7hhy6FvLqzc45sN5W6r4mu98;" -d '{"type": "create_card", "content": {"stack_id": "AX2JMjKIOr", "frontside": "dupa?", "backside": "tak!"}}' localhost:8000/api/v1/cards
- update_stack:
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTYxNTUxfQ.wz1kbm63J2Pzx61nXLn7hhy6FvLqzc45sN5W6r4mu98;" -d '{"type": "update_stack", "content": {"unique_id": "N4vGQ-FgS8", "name": "dupa?"}}' localhost:8000/api/v1/cards
- update_card:
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTYxNTUxfQ.wz1kbm63J2Pzx61nXLn7hhy6FvLqzc45sN5W6r4mu98;" -d '{"type": "update_card", "content": {"unique_id": "bVX1F5Rqgj", "frontside": "dupa?", "backside": "nie :3"}}' localhost:8000/api/v1/cards
- delete_stack:
  - curl -i -X POST -H "Content-Type: application/json" -H "Cookie: jwt_v1=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkcDFPbmJSRGVQIiwiZXhwIjoxNjk5MTYxNTUxfQ.wz1kbm63J2Pzx61nXLn7hhy6FvLqzc45sN5W6r4mu98;" -d '{"type": "delete_stack", "content": {"unique_id": "bVX1F5Rqgj"}}' localhost:8000/api/v1/cards
  
*/