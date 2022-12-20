<script lang="ts">
    export let text = "Default text";
    // clean text string prop to remove new line characters
    // at the beginning of the string
    // let text_stripped = text.replace(/^\n+/, "");
</script>

<div class="message-sent-box">
    <p class="message-text">{text}</p>
</div>
<style>
.message-sent-box {
    /* background-color: #d0a383; */
    display: flex;
    align-items: flex-end;
    margin-bottom: 10px;
    float: left;
    }

    .message-text {
    background-color: violet;
    padding: 10px;
    border-radius: 10px;
    max-width: 100%;
    }
</style>