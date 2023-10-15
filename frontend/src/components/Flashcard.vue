<template>
    <div class="flashcard">
        <div class="question">{{ card.question }}</div>
    <div class="answer">{{ card.answer }}</div>
    </div>
</template>
  
<script>
export default {
    props: ['card'],
};
</script>
  