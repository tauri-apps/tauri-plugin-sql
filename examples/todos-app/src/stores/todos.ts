import { writable, derived } from 'svelte/store';
import Storage from '../services/Storage';
import type { Todo, NewTodo, ExistingTodo } from '../models/Todo';

export const todos = writable([]);

export async function init(): Promise<void> {
  const $todos: Todo[] = await Storage.all();
  todos.set($todos);
}

export async function add({ detail }: CustomEvent<NewTodo>): Promise<void> {
  const newTodo: Todo = await Storage.create(detail.title);

  todos.update(($todos) => {
    return [...$todos, newTodo];
  });
}

export async function toggleComplete({ detail }: CustomEvent<ExistingTodo>): Promise<void> {
  todos.update(($todos) => {
    const todo: Todo | undefined = $todos.find((todo) => todo.id === detail.id);
    if (!todo) return;

    todo.completed = !todo.completed;
    Storage.update(todo);
    return $todos;
  });
}

export async function remove({ detail }: CustomEvent<ExistingTodo>): Promise<void> {
  todos.update(($todos) => {
    const index = $todos.findIndex((todo) => todo.id === detail.id);
    $todos.splice(index, 1);
    Storage.remove(detail.id);
    return $todos;
  });
}

export const incompleteCount = derived(
  todos,
  ($todos) => $todos.filter((todo) => !todo.completed).length
);
