<script>
  import Flashcard from "./Flashcard.svelte";

  import Menu from "./Menu.svelte";

  let menuItems = [
    { label: 'Home', link: '/' },
    { label: 'About', link: '/about' },
    { label: 'Services', link: '/services' },
    { label: 'Contact', link: '/contact' }
  ];
</script>


<Menu {menuItems} />
<Flashcard />



