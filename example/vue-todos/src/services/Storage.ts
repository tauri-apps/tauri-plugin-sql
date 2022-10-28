import Database from 'tauri-plugin-sql-api';
import type { QueryResult } from 'tauri-plugin-sql-api';
import { v4 } from 'uuid';
import type { Todo, uuid } from '../models/Todo';

let database: Database;

export async function connect(): Promise<Database> {
  if (database) {
    return database;
  } else {
    database = await Database.load('sqlite:test.db');
    return database;
  }
}

export async function all(): Promise<Todo[]> {
  const db = await connect();

  return await db.select('SELECT * FROM todos');
}

export async function create(title: string): Promise<Todo> {
  const db = await connect();

  const newTodo = {
    id: v4(),
    title,
    completed: false
  };
  if (db) {
    await db.execute('INSERT INTO todos (id, title, completed) VALUES ($1,$2,$3)', [
      newTodo.id,
      title,
      false
    ]);
  } else {
    console.warn(`There is not a valid DB connection, adding TODO to local storage only`);
  }
  return newTodo;
}

export async function select<T = unknown>(query: string): Promise<T> {
  const db = await connect();

  return db.select(query);
}

export async function execute(query: string): Promise<QueryResult> {
  const db = await connect();

  return db.execute(query);
}

export async function update(todo: Todo): Promise<Todo> {
  const db = await connect();

  await db.execute('UPDATE todos SET title = $1, completed = $2 WHERE id = $3', [
    todo.title,
    todo.completed,
    todo.id
  ]);
  return todo;
}

export async function remove(id: uuid): Promise<QueryResult> {
  const db = await connect();

  return await db.execute('DELETE FROM todos WHERE id = $1', [id]);
}
