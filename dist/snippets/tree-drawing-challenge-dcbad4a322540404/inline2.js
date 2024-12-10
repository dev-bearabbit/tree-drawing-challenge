
export function copyToClipboard(text) {
    navigator.clipboard.writeText(text)
        .then(() => {
            console.log('copy complete:', text);
        })
        .catch(err => {
            console.error('copy failed:', err);
        });
}
