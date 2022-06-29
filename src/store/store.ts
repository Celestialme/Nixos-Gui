import {writable} from 'svelte/store'
export const currentScreen = writable(0)
export const optionList = writable({})
export const OptionInputValue = writable('')
export const ast = writable()
export const needsSaving =  writable(false)
export const needs_db_update =  writable(false)
export const changes =  writable({})
export const markedPkgs = writable([])
export const nixEnvPkgs = writable([])

