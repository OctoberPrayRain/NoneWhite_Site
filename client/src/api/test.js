export async function fetchTestStatus() {
  const response = await fetch('/api/test')

  if (!response.ok) {
    throw new Error(`请求失败：${response.status}`)
  }

  return response.json()
}
