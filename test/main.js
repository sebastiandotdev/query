/**
 * Playground for query with Deno
 * Github: https://github.com/castrogarciajs/query
 * 2023 - Sebastian Castro - MIT License
 */

import { Application, oakCors } from './deps.js'
import router from './routes.js'

const app = new Application()

app.use(oakCors()) // Enable CORS for All Routes
app.use(router.routes())
app.use(router.allowedMethods())

console.log({ example: 'http://localhost:8000' })
await app.listen({ port: 8000 })
