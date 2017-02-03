import './main.scss';
import Rx from 'rxjs/Rx';
let currentDay = 1;

(function() {app();})();

function app() {
    let body = document.body;
    let fragment = document.createDocumentFragment();
    let wrapper = document.createElement('div');
    wrapper.innerHTML = template(currentDay);
    fragment.appendChild(wrapper);
    body.appendChild(fragment);

    let increment = Rx.Observable.fromEvent(document.getElementById('increment'), 'click');
    let decrement = Rx.Observable.fromEvent(document.getElementById('decrement'), 'click');
    let day = document.getElementById('day');
    let timer = document.getElementById('timer');
    let startTimerButton = Rx.Observable.fromEvent(document.getElementById('start-timer'), 'click');

    increment.subscribe(e => incrementDay(currentDay, day));
    decrement.subscribe(e => decrementDay(currentDay, day));
    startTimerButton.subscribe(e => startTimer(currentDay, timer));
}

function startTimer(day, timer) {
    let minutes = 60;
    let checkInterval = 1;
    let sequence = [
        [3, 5, 10],
        [5, 10, 12],
        [10, 12, 15],
        [12, 15, 17],
        [15, 17, 20],
        [17, 20, 25],
        [20, 25, 30]
    ];

    let timeSequence = sequence[--day].reduce((acc, seq) => {
        acc.push(
            Rx.Observable.timer(1, 1000)
            .take(seq*minutes)
        );
        acc.push(
            Rx.Observable.timer(1, 1000)
                .take(checkInterval*minutes)
        );
        return acc;
    }, []);

    Rx.Observable.from(timeSequence)
        .concatMap(obs => obs)
        .subscribe(second => updateTimer(timer, second, 1));
}

function updateTimer(timer, time, session) {
    // Time is in seconds so format it
    let formatTime = new Date(time * 1000).toISOString().substr(11, 8);
    timer.innerHTML = `<div>${formatTime}</div>`;
    return time;
}

function incrementDay(originalDay, dayElement) {
    currentDay = ++ originalDay;
    dayElement.innerHTML = `Day ${currentDay}`;
}

function decrementDay(originalDay, dayElement) {
    currentDay = --originalDay ? originalDay : 1;
    dayElement.innerHTML = `Day ${currentDay}`;
}

function template(day) {
    return `
        <div id="day-selector">
            <div id="day">Day ${day}</div>
            <div id="day-editor">
                <div id="increment">&#8593;</div>
                <div id="decrement">&#8595;</div>
            </div>
        </div>
        <div id="timer">00:00:00</div>
        <button id="start-timer">Start Timer</button>
    `
}
