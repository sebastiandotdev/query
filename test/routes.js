import { Router } from './deps.js'
import data from './data.json' assert { type: 'json' }

const router = new Router()

router
  .get('/', (ctx) => {
    ctx.response.body = 'Welcome to dinosaur API!'
  })
  .get('/api', (ctx) => {
    ctx.response.body = data
  })
  .get('/api/:dinosaur', (ctx) => {
    if (ctx?.params?.dinosaur) {
      const found = data.find(
        (item) => item.name.toLowerCase() === ctx.params.dinosaur.toLowerCase(),
      )
      if (found) {
        ctx.response.body = found
      } else {
        ctx.response.body = 'No dinosaurs found.'
      }
    }
  }).post('/api', (ctx) => {
    ctx.response.body = 'Created new dinosaur'
    console.log(ctx)
  }).delete('/api/:id', (ctx) => {
    ctx.response.body = 'Deleted dinosaur'
    console.log(ctx)
  })
export default router
