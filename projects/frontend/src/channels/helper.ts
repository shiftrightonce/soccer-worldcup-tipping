const firstChannelInstance: Record<string, boolean> = {}
export const makeChannel = (name: string, callback?: (channel: BroadcastChannel) => void) => {
  firstChannelInstance[name] = firstChannelInstance[name] || true

  const channel = new BroadcastChannel(name)

  if (firstChannelInstance[name]) {
    firstChannelInstance[name] = false
    channel.addEventListener('message', (event) => {
      if (event.type && event.data) {
        dispatchEvent(new CustomEvent(`${name}:${event.type}`, { detail: event }))
      }
    })
  }

  if (callback) {
    callback(channel)
  }

  return channel
}
