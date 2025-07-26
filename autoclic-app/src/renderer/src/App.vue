<script setup>
import { ref, computed, onMounted } from 'vue'
import CounterClick from './components/CounterClick.vue'
import ControlButtons from './components/ControlButtons.vue'

const cps = ref()
const isRunning = ref(false)

const containerStyle = computed(() => ({
  backgroundColor: isRunning.value ? 'orange' : 'var(--color-background)',
  padding: '10px',
  height: 'calc(100% - 100px)'
}))

onMounted(() => {
  window.api.receive('running', (value) => {
    isRunning.value = value
  })
  window.api.getCps().then((value) => {
    cps.value = value
  })
})

function updateCps() {
  window.api.setCps(cps.value)
}

function handleStart() {
  window.electron.ipcRenderer.send('start')
}

function handleStop() {
  window.electron.ipcRenderer.send('stop')
}
</script>

<template>
  <div :style="containerStyle">
    <div style="margin-bottom: 10px">
      Démarrer :
      <kbd><kbd>⌘ Command</kbd> ou <kbd>Ctrl</kbd></kbd> + <kbd>Alt</kbd> + <kbd>A</kbd>
    </div>
    <div style="margin-bottom: 10px">Stop : <kbd>Q</kbd></div>
    <div>
      1 clique toutes les
      <input v-model.number="cps" type="number" min="0" @input="updateCps" />
      millisecondes
    </div>
    <ControlButtons :is-running="isRunning" @start="handleStart" @stop="handleStop" />
  </div>
  <CounterClick />
</template>
