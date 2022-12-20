<script lang="ts">
  export let text = "Default text";
</script>
<div class="message-sent-box">
    <p class="message-text">{text}</p>
    <!-- <p class="message-timestamp">10:15 AM</p> -->
    <!-- <p>Sample text</p> -->
  </div>
<style>
.message-sent-box {
    /* background-color: #edd9cb; */
    display: flex;
    align-items: flex-end;
    margin-bottom: 10px;
    float: right;
    }

    .message-text {
    background-color: lightblue;
    padding: 10px;
    border-radius: 10px;
    max-width: 100%;
    }

    /* .message-timestamp {
    font-size: smaller;
    color: gray;
    margin-left: 5px;
    } */
</style>