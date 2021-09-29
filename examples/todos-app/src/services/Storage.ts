import Database from 'tauri-plugin-sql-api';
import type { Todo } from '../models/Todo';

let db = null
Database.load('sqlite:test.db').then(instance => {
  db = instance
})

async function all(): Promise<Todo[]> {
  return await db.select('SELECT * FROM todos');
}

async function create(title: string): Promise<Todo> {
  await db.execute(`INSERT INTO todos (title) VALUES ('${title}')`);
  return {
    id: 1,
    title,
    completed: false,
  };
}

async function update(todo: Todo): Promise<Todo> {
  await db.execute(`UPDATE todos SET title = '${todo.title}', completed = ${todo.completed} WHERE id = ${todo.id}`);
  return todo;
}

async function remove(id: number): Promise<boolean> {
  return await db.execute(`DELETE FROM todos WHERE id = ${id}`);
}

export default {
  all,
  create,
  update,
  remove
};
