# REST-API-using-Rust
A simple REST API application, implemented in rust programming language. The four API's (GET, POST, PUT, DELETE) are implemented. MongoDB is used as database.

Application takes information for a movie and calculates Verdict of that Movie.

Features :-
1. Input and Business Validations are done.
2. Error Handling is done.
3. Rolling JSON logs are provided.


Please refer HELP folder for any kind of help.

------
To run application.
Refer HELP\Application_Requirements.txt for system requirements.

Clone the repository.

Download MongoDB community server zip package for windows.
Extract ZIP.
Create a folder to store database.
We will call this folder as 'db'
Go to bin folder of MongoDB.
bin folder is located at the location where we have extracted MongoDB community server zip package.  
Execute following command inside bin folder from command prompt.
mongod --dbpath="<Path of 'db' folder>" 
Above steps will start MongoDB Server.


Use MongoDB Compass to import data.
Download and extract MongoDBCompass and run MongoDBCompass.exe
Enter connection string for database server running locally.
mongodb://localhost

Create Database and Collection and then Import Data using MongoDBCompass.

Refer Help\DatabaseHelp folder for Database Schema and Collection. 

In VS Code Open Thunder Client, select Collections.
Click Menu at the right side and select Import.
Import JSON requests collection.
This collection is present in file 'thunder-collection_MoviesJSON.json'. 

Build and Run

cargo build

cargo run

Send requests from Thunder Client.



