# Flashcard Website Project - ToDo LIST

## Overall Project Goals

- [ ] Deploy a fully functional flashcard website

## Project Structure

flashcard-website/  
│  
├── frontend/  
│ ├── src/  
│ │ ├── components/  
│ │ │ ├── CardList.vue  
│ │ │ ├── Flashcard.vue  
│ │ ├── App.vue  
│ │ ├── main.js  
│ ├── package.json  
│  
├── backend/  
│ ├── src/  
│ │ ├── main.rs  
│ │ ├── routes.rs  
│ │ ├── models.rs  
│ │ ├── db.rs  
│ ├── Cargo.toml  
│  
├── .env  
└── README.md  

## Frontend (Vue.js)

- [x] Create a Vue.js project
- [ ] Setup some HTTP request lib like Axios
- [ ] Build comps and all those stuff (editable)
- [ ] Connect to Rust backend

## Backend (Rust)

- [x] Create a new Rust project
- [x] Setup database connection
- [ ] Create API routes
- [ ] Implement the flashcard routes

## Database (MySQL)

- [x] Create a database 
- [x] Update .env file with MySQL database connection
- [x] Setup all database stuff
