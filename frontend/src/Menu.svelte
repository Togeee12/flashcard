<script>
    import { listen } from "svelte/internal";

    export let menuItems = [];

    function navigate(link) {
        console.log("Navigating to link: {link}")
    }

</script>

<nav>
    <ul>
        {#each menuItems as item (item.label)}
      <li>
        <a on:click={() => navigate(item.link)}>{item.label}</a>
      </li>
    {/each}
    </ul>
</nav>


<style>
ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

nav {
    background-color: #eee;
    padding: 10px; 
}

ul {
    display: flex; 
    justify-content: space-between; 
}

li {
    margin: 0 15px;     

}

a {
    text-decoration: none; 
    color: #fff;
    font-weight: bold; 
    transition: color 0.3s;
    color: #42004b;
}

a:hover {
    color: #6e007c; 
    cursor: pointer;
}

</style>