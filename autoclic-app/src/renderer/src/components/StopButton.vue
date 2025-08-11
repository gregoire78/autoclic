<script setup>
defineProps({
  isDisabled: Boolean
})
const emit = defineEmits(['stop'])

function handleStop() {
  emit('stop')
}
</script>

<template>
  <button :disabled="isDisabled" style="width: 25%" @click="handleStop">Arrêter</button>
</template>
