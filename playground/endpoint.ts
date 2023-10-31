import { Router } from './deps.ts'
import data from './data.json' assert { type: 'json' }

const router = new Router()

router
  .get('/', (context) => {
    context.response.body = 'Welcome to dinosaur API!'
  })
  .get('/api', (context) => {
    context.response.body = data
  })
  .get('/api/:dinosaur', (context) => {
    if (context?.params?.dinosaur) {
      const found = data.find(
        (item) =>
          item.name.toLowerCase() === context.params.dinosaur.toLowerCase(),
      )
      if (found) {
        context.response.body = found
      } else {
        context.response.body = 'No dinosaurs found.'
      }
    }
  })

export default router
