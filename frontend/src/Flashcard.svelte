<script>
  let message = "";

  async function fetchData() {
    try {
      const response = await fetch("/api/hello");
      const data = await response.json();
      message = data;
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  let isFlipped = false;

  function toggleCard() {
    isFlipped = !isFlipped;
  }

  // Here we will instert a data from Api call to a card!
  let frontKeyWord = "";
  let backKeyWord = "";
</script>

<div>
  <button on:click={fetchData}>Fetch Data</button>
  <p>{message}</p>

  <div class="flashcard" class:flipped={isFlipped} on:click={toggleCard}>
    <div class="front">
      <slot name="front">Front Content</slot>
    </div>
    <div class="back">
      <slot name="back">Back Content</slot>
    </div>
  </div>
</div>

<style>
  .flashcard {
    width: 200px;
    height: 150px;
    perspective: 1000px;
    position: relative;
  }

  .front,
  .back {
    width: 100%;
    height: 100%;
    position: absolute;
    backface-visibility: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .back {
    transform: rotateY(180deg);
  }

  .flipped .front {
    transform: rotateY(180deg);
  }

  .flipped .back {
    transform: rotateY(0deg);
  }
</style>
