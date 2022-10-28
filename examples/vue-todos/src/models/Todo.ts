export type uuid = string;

export interface Todo {
  id: uuid;
  title: string;
  completed: boolean;
}

export interface NewTodo {
  title: string;
}

export interface ExistingTodo {
  id: uuid;
}
