<script>
  import { Router, Link, Route } from "svelte-routing";
  import Room from "./routes/Room.svelte";
  import Join from "./routes/Join.svelte";

  export let url = "";
</script>

<Router url="{url}">
  <nav>
    <a href="/">Home</a>
  </nav>
  <div>
    <Route path="/"><Room /></Route>
    <Route path="/join/:id/" let:params>
        <Join room_id="{params.id}" />
    </Route>
  </div>
</Router>
