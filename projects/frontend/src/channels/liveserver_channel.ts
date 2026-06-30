import { makeChannel } from './helper'

export const CHANNEL_NAME = 'live_server'

export const makeLiveServerChannel = (callback?: (channel: BroadcastChannel) => void) => {
  return makeChannel(CHANNEL_NAME, callback)
}

export default makeLiveServerChannel
