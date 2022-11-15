import { writable } from "svelte/store";
import Database, { type QueryResult } from 'tauri-plugin-sql-api';

export type Todo = {
    title: string;
    id: number;
    completed: boolean;
}

const createTodoStore = async () => {
    const { subscribe, set, update} = writable<Todo[]>([]);

    // New database connection
    let db = await Database.load('sqlite:db.sqlite')
    // Migrations
    await db.execute("CREATE TABLE IF NOT EXISTS todo (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, title VARCHAR(200) NOT NULL, completed BOOLEAN DEFAULT FALSE);");

    const fetchAll = async () => {
        const allTodos =  await db.select('SELECT * FROM todo') as Todo[]
        set(allTodos)
    }
    await fetchAll()

    return {
        subscribe,
        set,
        create: async (title: string): Promise<QueryResult> => {
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

export const todoStore = await createTodoStore()


