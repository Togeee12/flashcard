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
</script>

<div>
  <button on:click={fetchData}>Fetch Data</button>

  <p>{message}</p>
</div>
