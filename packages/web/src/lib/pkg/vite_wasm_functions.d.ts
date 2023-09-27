/* tslint:disable */
/* eslint-disable */
/**
* @returns {Promise<any>}
*/
export function getTodos(): Promise<any>;
/**
* @param {string} task
*/
export function addTodo(task: string): void;
/**
* @param {string} id
*/
export function deleteTodo(id: string): void;
/**
*/
export function markAllComplete(): void;
/**
*/
export function markAllActive(): void;
/**
* @param {string} id
*/
export function toggleComplete(id: string): void;
/**
*/
export function init(): void;
/**
* @returns {boolean}
*/
export function checkAllComplete(): boolean;
/**
*/
export class Todo {
  free(): void;
}
/**
*/
export class Todos {
  free(): void;
}
