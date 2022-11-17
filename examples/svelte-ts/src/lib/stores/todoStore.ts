import { writable } from "svelte/store";
import Database, { type QueryResult } from 'tauri-plugin-sql-api';

export type Todo = {
    title: string;
    id: number;
    completed: boolean;
}

const createTodoStore = () => {
    const { subscribe, set} = writable<Todo[]>([]);

    let db: Database; 

    // Run migrations on the connected database
    // You may also chose to run your migrations as a chain of UP or DOWN migrations in your rust code
    // Look at the src-tauri/src/main.rs file for an example
    const migrations = async () => 
        await db.execute("CREATE TABLE IF NOT EXISTS todo (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, title VARCHAR(200) NOT NULL, completed BOOLEAN DEFAULT FALSE);")

    // Here we are fetching all todos from the todo table, reloading the whole todo store in the process
    //
    // Note that this example reloads the todo store with all of the data from the database each time there is a crud operation
    // This is done for the sake of clarity, as we want to show what is currently stored in the database at all times
    const fetchAll = async () => {
        // The select method takes a generic parameter T and returns a type Promise<T>
        // This means that we can pass a <ExampleType[]> to the select method and it will returns us a Promise<ExampleType[]>
        // Keep in mind that it always returns an array of items
        const allTodos =  await db.select<Todo[]>('SELECT * FROM todo')
        // As it is impossible for typescript or rust to generate the types at runtime,
        // the user is trusted on this one to know the exact return shape of the data that was queried

        // For example, if you know you only want the title and id, here is how you'd do it
        // const allTodos =  await db.select<Pick<Todo, 'id'|"title" >[]>('SELECT id, title FROM todo')
        // console.log(allTodos?.[0].title)
        // console.log(allTodos?.[0].id)
        // console.log(allTodos?.[0].completed) // Err: Property completed  doesn't exist on object

        set(allTodos)
    }
    

    return {
        subscribe,
        init: async () => {
            // Load the database and create a new connection
            // The plugin supports the following SQL drivers: MySql, Postgres, and SQLITE
            // sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
            // ** - sqlite 
            // ** const db = await Database.load('sqlite:test.db')
            // ** - mysql
            // ** const db = await Database.load('mysql://user:pass@host/database')
            // ** - postgres
            // ** const db = await Database.load('postgres://postgres:password@localhost/test')
            db = await Database.load('sqlite:db.sqlite')
            await migrations()
            await fetchAll()
        },
        create: async (title: string): Promise<QueryResult> => {
            // The execute method returns a object with `rowsAffected` and `lastInsertId` properties 
            // Although we haven't used these return values in the example, do keep in mind that they are there if you need them
            const result = await db.execute('INSERT INTO todo (title) VALUES ($1)', [title]);
            await fetchAll()
            return result

        },
        update:async (todo:Todo):Promise<QueryResult> => {
            const result = await db.execute('UPDATE todo SET title = $1, completed = $2 WHERE id = $3', [todo.title, todo.completed, todo.id]);
            await fetchAll()
            return result

        },
        remove:async (id:number):Promise<QueryResult> => {
            const result = await db.execute('DELETE FROM todo WHERE id = $1', [id]);
            await fetchAll()
            return result
        }
    }
}

export const todoStore =  createTodoStore()


