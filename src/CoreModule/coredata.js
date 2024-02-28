const invoke = window.__TAURI__.invoke;

let datas;
invoke('prodata_read').then(result => {
    console.log(result);
    datas = result || {
        colormode: 'auto',
    };
    switch (datas.colormode) {
        case 'light':
            preConfig1.classList.remove('active')
            config1.children[0].classList.add('active')
            preConfig1 = config1.children[0];
            colormode_switch('light');
            break;
        case 'dark':
            preConfig1.classList.remove('active')
            config1.children[2].classList.add('active')
            preConfig1 = config1.children[2];
            colormode_switch('dark');
            break;
        default:
            preConfig1.classList.remove('active')
            config1.children[1].classList.add('active')
            preConfig1 = config1.children[1];
            if (matchMedia('(prefers-color-scheme: dark)').matches)
                colormode_switch('dark')
            else colormode_switch('light')
            break;
    }
})

const config1 = document.getElementById('config1')
let preConfig1 = config1.children[1];
Array.from(config1.children).forEach(item => {
    item.addEventListener('click', e => {
        if (preConfig1 != e.target) {
            preConfig1.classList.remove('active')
            preConfig1.style.backgroundColor = null
            e.target.classList.add('active')
            e.target.style.backgroundColor = '#396cd8'
            preConfig1 = e.target;

            switch (Array.from(config1.children).indexOf(e.target)) {
                case 0:
                    datas.colormode = 'light'
                    colormode_switch('light');
                    break;
                case 2:
                    datas.colormode = 'dark'
                    colormode_switch('dark');
                    break;
                case 1:
                    datas.colormode = 'auto'
                    if (matchMedia('(prefers-color-scheme: dark)').matches)
                        colormode_switch('dark')
                    else colormode_switch('light')
                    break;
            }
            invoke('prodata_save', { configs: datas }).then(result => {
                console.log('Save result:' + result);
            })
        }
    })
})

// Switch colormode
function colormode_switch(mode) {
    if (mode == 'light') {
        document.documentElement.style.fontWeight = '400'
        document.documentElement.style.color = '#0f0f0fd6'
        document.documentElement.style.backgroundColor = '#f6f6f66d'
        document.body.classList.remove('dark')
    }
    else if (mode == 'dark') {
        document.documentElement.style.fontWeight = '300'
        document.documentElement.style.color = '#f6f6f6d6'
        document.documentElement.style.backgroundColor = '#0f0f0fd6'
        document.body.classList.add('dark')
    }
}

if (matchMedia('(prefers-color-scheme: dark)').matches) {
    colormode_switch('dark')
}

// Auto change colormode
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
    // check root var
    if (e.matches && getComputedStyle(document.documentElement).getPropertyValue('--autocolormode')) {
        colormode_switch('dark')
    }
})
window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', e => {
    // check root var
    if (e.matches && getComputedStyle(document.documentElement).getPropertyValue('--autocolormode')) {
        colormode_switch('light')
    }
})