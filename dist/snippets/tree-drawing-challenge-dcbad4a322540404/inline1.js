
function uploadToImgBB(dataUrl) {
    const apiKey = '2fc4f7a32019bd384305c71135034668';
    const base64Image = dataUrl.split(',')[1];

    const formData = new FormData();
    formData.append('key', apiKey);
    formData.append('image', base64Image);

    return fetch('https://api.imgbb.com/1/upload', {
        method: 'POST',
        body: formData,
    }).then(response => response.json());
}

export { uploadToImgBB };
