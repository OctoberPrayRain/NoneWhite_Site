import { requestJson } from './http'

export async function fetchTestStatus() {
  return requestJson('/api/test', {
    returnEnvelope: true,
  })
}
