document.addEventListener('DOMContentLoaded', function () {
    const accordionItems = document.querySelectorAll('.accordion-item');

    accordionItems.forEach(item => {
        const trigger = item.querySelector('.accordion-trigger');

        trigger.addEventListener('click', () => {
            const isActive = item.classList.contains('active');

            // Close all accordion items
            accordionItems.forEach(accItem => {
                accItem.classList.remove('active');
            });

            // Toggle the clicked item
            if (!isActive) {
                item.classList.add('active');
            }
        });
    });
});

document.addEventListener('DOMContentLoaded', function () {
    const tabs = document.querySelectorAll('.flow-tab');
    const contents = document.querySelectorAll('.flow-content');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // Remove active class from all tabs and contents
            tabs.forEach(t => t.classList.remove('active'));
            contents.forEach(c => c.classList.remove('active'));

            // Add active class to clicked tab and corresponding content
            const tabIndex = tab.dataset.tab;
            tab.classList.add('active');
            document.querySelector(`.flow-content[data-content="${tabIndex}"]`).classList.add('active');
        });
    });
});
