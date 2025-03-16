document.addEventListener("DOMContentLoaded", () => {
    const editor = document.getElementById("editor");
    const wordCount = document.getElementById("word-count");

    // Load saved text from Local Storage
    editor.innerHTML = localStorage.getItem("editorContent") || "<p>Start typing...</p>";

    // Function to update word count
    function updateWordCount() {
        const text = editor.innerText.trim();
        const words = text.length > 0 ? text.split(/\s+/).length : 0;
        wordCount.innerText = `Words: ${words} | Characters: ${text.length}`;
    }

    // Save content to Local Storage
    function saveContent() {
        localStorage.setItem("editorContent", editor.innerHTML);
    }

    // Formatting function using execCommand
    window.formatText = (command) => {
        document.execCommand(command, false, null);
        editor.focus();
    };

    // Event listeners
    editor.addEventListener("input", () => {
        updateWordCount();
        saveContent();
    });

    updateWordCount();
});
