(function() {
    const mapping = {
        "blockquote-tag-note": "信息",
        "blockquote-tag-tip": "技巧",
        "blockquote-tag-important": "重要",
        "blockquote-tag-warning": "注意",
        "blockquote-tag-caution": "警告",
    };
    for (const key in mapping) {
        const admonitions = document.querySelectorAll(`blockquote.${key} > p.blockquote-tag-title`);
        admonitions.forEach((n) => {
            n.lastChild.textContent = mapping[key];
        });
    }
})()
