<template>
    <div>
      <div v-for="card in cards" :key="card.id">
        <Flashcard :card="card" />
      </div>
    </div>
</template>
  
<script>
export default {
    props: ['cards'],
};
</script>

  