/* Reusable quiz widgets for lessons in this workspace.
 *
 * Multiple choice — markup contract:
 *
 *   <div class="quiz" data-mcq data-answer="2">
 *     <div class="q">Question text</div>
 *     <div class="choices">
 *       <button>Choice A</button>
 *       <button>Choice B</button>
 *     </div>
 *     <div class="feedback"
 *          data-ok="Shown when correct"
 *          data-bad="Shown when wrong"></div>
 *   </div>
 *
 * data-answer is the 1-based index of the correct button. Wrong answers stay
 * clickable so the learner can keep retrieving until they get it; the correct
 * one locks the question.
 *
 * Free text — same markup but data-text with data-accept as a JSON array of
 * accepted substrings (whitespace-insensitive).
 */

document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('.quiz[data-mcq]').forEach((quiz) => {
    const answer = Number(quiz.dataset.answer);
    const feedback = quiz.querySelector('.feedback');
    const buttons = [...quiz.querySelectorAll('.choices button')];

    buttons.forEach((button, i) => {
      button.addEventListener('click', () => {
        const correct = i + 1 === answer;
        buttons.forEach((b) => b.classList.remove('picked-ok', 'picked-bad'));
        button.classList.add(correct ? 'picked-ok' : 'picked-bad');
        feedback.textContent = correct ? feedback.dataset.ok : feedback.dataset.bad;
        feedback.className = 'feedback ' + (correct ? 'ok' : 'bad');
        if (correct) buttons.forEach((b) => (b.disabled = true));
      });
    });
  });

  document.querySelectorAll('.quiz[data-text]').forEach((quiz) => {
    const accept = JSON.parse(quiz.dataset.accept);
    const input = quiz.querySelector('input[type="text"]');
    const feedback = quiz.querySelector('.feedback');

    const check = () => {
      const value = input.value.replace(/\s/g, '');
      const correct = value.length > 0 &&
        accept.some((a) => value.includes(a.replace(/\s/g, '')));
      feedback.textContent = correct ? feedback.dataset.ok : feedback.dataset.bad;
      feedback.className = 'feedback ' + (correct ? 'ok' : 'bad');
    };

    quiz.querySelector('button').addEventListener('click', check);
    input.addEventListener('keydown', (e) => { if (e.key === 'Enter') check(); });
  });
});
