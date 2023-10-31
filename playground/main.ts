/**
 * Playground for fetchy with Deno
 * Github: https://github.com/castrogarciajs/fetchy
 * 2023 - Sebastian Castro García - MIT License
 * @version 0.1.0
 */

import { Application, oakCors } from './deps.ts'
import router from './endpoint.ts'

const app = new Application()

app.use(oakCors()) // Enable CORS for All Routes
app.use(router.routes())
app.use(router.allowedMethods())

console.log({ example: 'http://localhost:8000' })
await app.listen({ port: 8000 })
