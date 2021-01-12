# Quickstart: Build a Rust application backed by Azure Cosmos DB's API for MongoDB

The sample presented in this article is a simple command-line based application that uses the [Rust driver for MongoDB](https://github.com/mongodb/mongo-rust-driver). Since Azure Cosmos DB's API for MongoDB is [compatible with the MongoDB wire protocol](./mongodb-introduction.md#wire-protocol-compatibility), it is possible for any MongoDB client driver to connect to it.

You will learn how to use the MongoDB Rust driver to interact with Azure Cosmos DB's API for MongoDB by exploring CRUD (create, read, update, delete) operations implemented in the sample code. Finally, you can run the application locally to see it in action.

## Pre-requisites

- An Azure account with an active subscription. [Create one for free](https://azure.microsoft.com/free/?WT.mc_id=data-12579-abhishgu). Or [try Azure Cosmos DB for free](https://azure.microsoft.com/try/cosmosdb/?WT.mc_id=data-12579-abhishgu) without an Azure subscription. You can also use the [Azure Cosmos DB Emulator](https://aka.ms/cosmosdb-emulator) with the connection string `.mongodb://localhost:C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==@localhost:10255/admin?ssl=true`.
- [Rust](https://www.rust-lang.org/tools/install) (version 1.39 or above)
- [Git](https://git-scm.com/downloads)

## Setup Azure Cosmos DB

You need to [create a database account](https://docs.microsoft.com/azure/cosmos-db/create-mongodb-dotnet?WT.mc_id=data-12579-abhishgu#create-a-database-account) for Azure Cosmos DB for Mongo DB API and retrieve the database key that will allow you to connect to the database in subsequent steps.

## Run the application

### Clone the sample application

Run the following commands to clone the sample repository.

1. Open a command prompt, create a new folder named `git-samples`, then close the command prompt.

    ```bash
    mkdir "C:\git-samples"
    ```

1. Open a git terminal window, such as git bash, and use the `cd` command to change to the new folder to install the sample app.

    ```bash
    cd "C:\git-samples"
    ```

1. Run the following command to clone the sample repository. This command creates a copy of the sample app on your computer. 

    ```bash
    git clone https://github.com/Azure-Samples/cosmosdb-rust-mongodb-quickstart
    ```

### Build the application

To build the binary:

```bash
cargo build --release
```

### Configure the application 

Export the connection string, MongoDB database, and collection names as environment variables. 

```bash
export MONGODB_URL="mongodb://<COSMOSDB_ACCOUNT_NAME>:<COSMOSDB_PASSWORD>@<COSMOSDB_ACCOUNT_NAME>.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&maxIdleTimeMS=120000&appName=@<COSMOSDB_ACCOUNT_NAME>@"
```

> The `ssl=true` option is important because of Cosmos DB requirements. For more information, see [Connection string requirements](https://docs.microsoft.com/azure/cosmos-db/connect-mongodb-account?WT.mc_id=data-12579-abhishgu#connection-string-requirements).
>

For the `MONGODB_URL` environment variable, replace the placeholders for `<COSMOSDB_ACCOUNT_NAME>` and `<COSMOSDB_PASSWORD>`

1. `<COSMOSDB_ACCOUNT_NAME>`: The name of the Azure Cosmos DB account you created
2. `<COSMOSDB_PASSWORD>`: The database key extracted in the previous step

```bash
export MONGODB_DATABASE=todos_db
export MONGODB_COLLECTION=todos
```

You can choose your preferred values for `MONGODB_DATABASE` and `MONGODB_COLLECTION` or leave them as is.

To run the application, change to the correct folder (where the application binary exists):

```bash
cd target/release
```

To create a `todo`

```bash
./todo create "Create an Azure Cosmos DB database account"
```

If successful, you should see an output with the MongoDB `_id` of the newly created document:

```bash
inserted todo with id = ObjectId("5ffd1ca3004cc935004a0959")
```

Create another `todo`

```bash
./todo create "Get the MongoDB connection string using the Azure CLI"
```

List all the `todo`s

```bash
./todo list all
```

You should see the ones you just added:

```bash
todo_id: 5ffd1ca3004cc935004a0959 | description: Create an Azure Cosmos DB database account | status: pending
todo_id: 5ffd1cbe003bcec40022c81c | description: Get the MongoDB connection string using the Azure CLI | status: pending
```

To update the status of a `todo` (for example, change it to `completed` status), use the `todo` ID as such:

```bash
./todo update 5ffd1ca3004cc935004a0959 completed

#output
updating todo_id 5ffd1ca3004cc935004a0959 status to completed
updated status for todo id 5ffd1ca3004cc935004a0959
```

List only the completed `todo`s

```bash
./todo list completed
```

You should see the one you just updated

```bash
listing 'completed' todos

todo_id: 5ffd1ca3004cc935004a0959 | description: Create an Azure Cosmos DB database account | status: completed
```

Delete a `todo` using it's ID

```bash
./todo delete 5ffd1ca3004cc935004a0959
```

List the `todo`s to confirm

```bash
./todo list all
```

The `todo` you just deleted should not be present

### View data in Data Explorer

Data stored in Azure Cosmos DB is available to view and query in the Azure portal.

To view, query, and work with the user data created in the previous step, login to the [Azure portal](https://portal.azure.com) in your web browser.

In the top Search box, enter **Azure Cosmos DB**. When your Cosmos account blade opens, select your Cosmos account. In the left navigation, select **Data Explorer**. Expand your collection in the Collections pane, and then you can view the documents in the collection, query the data, and even create and run stored procedures, triggers, and UDFs. 

## Clean up resources

When you're done with your app and Azure Cosmos DB account, [you can delete the Azure resources](https://docs.microsoft.com/en-us/azure/cosmos-db/create-mongodb-dotnet#clean-up-resources) you created so you don't incur more charges. To delete the resources:

## Resources

In this quickstart, you learned how to create an Azure Cosmos DB MongoDB API account using the Azure Cloud Shell, and create and run a Rust command-line app to manage `todo`s. You can now import additional data to your Azure Cosmos DB account. You can also explore the following resources:

- [Pre-migration steps for data migrations from MongoDB to Azure Cosmos DB's API for MongoDB](https://docs.microsoft.com/azure/cosmos-db/mongodb-pre-migration?WT.mc_id=data-12579-abhishgu)
- [Upgrade the MongoDB wire protocol version of your Azure Cosmos DB's API for MongoDB account](https://docs.microsoft.com/azure/cosmos-db/mongodb-version-upgrade?WT.mc_id=data-12579-abhishgu)
- [Quickstart: Connect a Go application to Azure Cosmos DB's API for MongoDB](https://docs.microsoft.com/azure/cosmos-db/create-mongodb-go?WT.mc_id=data-12579-abhishgu)
- [Consistency levels for Azure Cosmos DB and the API for MongoDB](https://docs.microsoft.com/azure/cosmos-db/mongodb-consistency?WT.mc_id=data-12579-abhishgu)