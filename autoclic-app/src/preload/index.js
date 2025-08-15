import { contextBridge, ipcRenderer } from 'electron'
import { electronAPI } from '@electron-toolkit/preload'

// Custom APIs for renderer
const api = {
  setCps: async (cps) => {
    return await ipcRenderer.invoke('setCps', cps)
  },
  getCps: async () => {
    return await ipcRenderer.invoke('getCps')
  },
  getMousePosition: async () => {
    return await ipcRenderer.invoke('getMousePosition')
  },
  setMousePosition: async (position) => {
    return await ipcRenderer.invoke('setMousePosition', position)
  },
  receive: (channel, func) => {
    let validChannels = ['running']
    if (validChannels.includes(channel)) {
      // Deliberately strip event as it includes `sender`
      ipcRenderer.on(channel, (event, ...args) => func(...args))
    }
  }
}

// Use `contextBridge` APIs to expose Electron APIs to
// renderer only if context isolation is enabled, otherwise
// just add to the DOM global.
if (process.contextIsolated) {
  try {
    contextBridge.exposeInMainWorld('electron', electronAPI)
    contextBridge.exposeInMainWorld('api', api)
  } catch (error) {
    console.error(error)
  }
} else {
  window.electron = electronAPI
  window.api = api
}
