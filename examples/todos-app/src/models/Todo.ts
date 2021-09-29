export interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

export interface NewTodo {
  title: string;
}

export interface ExistingTodo {
  id: number;
}
