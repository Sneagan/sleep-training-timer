import './main.scss';
import Rx from 'rxjs/Rx';
let currentDay = 1;

(function() {app();})();

function app() {
    let body = document.body;
    let fragment = document.createDocumentFragment();
    let wrapper = document.createElement('div');
    wrapper.id = 'wrapper';
    wrapper.innerHTML = template(currentDay);
    fragment.appendChild(wrapper);
    body.appendChild(fragment);

    let increment = Rx.Observable.fromEvent(document.getElementById('increment'), 'click');
    let decrement = Rx.Observable.fromEvent(document.getElementById('decrement'), 'click');
    let day = document.getElementById('day');
    let timer = document.getElementById('timer');
    let messageContainer = document.getElementById('message');
    let startTimerButton = Rx.Observable.fromEvent(document.getElementById('start-timer'), 'click');

    increment.subscribe(e => incrementDay(currentDay, day));
    decrement.subscribe(e => decrementDay(currentDay, day));
    startTimerButton.subscribe(e => startTimer(currentDay, timer, messageContainer));
}

function startTimer(day, timer, messageContainer) {
    let minutes = 60;
    let checkInterval = 1;
    let maxTime = 0;
    let message = '';
    // Ferber intervals by day
    let sequence = [
        [3, 5, 10, 10, 10, 10],
        [5, 10, 12, 12, 12, 12],
        [10, 12, 15, 15, 15, 15],
        [12, 15, 17, 17, 17, 17],
        [15, 17, 20, 20, 20, 20],
        [17, 20, 25, 25, 25, 25],
        [20, 25, 30, 30, 30, 30]
    ];

    --day;
    if (day > sequence.length - 1) {
        day = sequence.length - 1;
    }

    let timeSequence = sequence[day].reduce((acc, seq) => {
        acc.push(
            Rx.Observable.timer(1, 1000)
                .map(item => {
                    maxTime = seq * minutes;
                    message = 'Do not respond to cries for the duration.';
                    return item;
                })
                .take(seq * minutes)
        );
        acc.push(
            Rx.Observable.timer(1, 1000)
                .map(item => {
                    maxTime = checkInterval * minutes;
                    message = 'Time to reassure.';
                    return item;
                })
                .take(checkInterval * minutes)
        );
        return acc;
    }, []);

    Rx.Observable.from(timeSequence)
        .concatMap(obs => obs)
        .subscribe(second => updateTimer(timer, maxTime - second, messageContainer, message), failure, vibrateCompletion);
}

function updateTimer(timer, time, messenger, sessionMessage) {
    // Time is in seconds so format it
    let formatTime = new Date(time * 1000).toISOString().substr(11, 8);
    timer.innerHTML = `<div>${formatTime}</div>`;
    messenger.innerHTML = `<div>${sessionMessage}</div>`;
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
        <h3>Sleep Training Timer</h3>
        <p>Timer will alternate between waiting and comforting periods.</p>
        <div class="timer-wrapper">
            <div id="day-selector">
                <div id="day">Day ${day}</div>
                <div id="day-editor">
                    <div id="increment">&#8593;</div>
                    <div id="decrement">&#8595;</div>
                </div>
            </div>
            <div id="timer">00:00:00</div>
        </div>
        <div id="message"></div>
        <button id="start-timer">Start Timer</button>
    `
}

function failure(err) {
    console.trace(err);
}

function vibrateCompletion() {
    if (window.navigator && window.navigator.vibrate) {
        window.navigator.vibrate(200, 50, 50, 200);
    }
}