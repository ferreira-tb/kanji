import { get, post } from '@/api/http';

export async function createBookmark(snippet: Snippet) {
  const response = await post('create-bookmark', { snippet });
  const data: BookmarkId = await response.json();
  return data;
}

export async function createQuiz(kanjis: readonly KanjiChar[]) {
  const response = await post('create-quiz', { kanjis });
  const data: Quiz = await response.json();
  return data;
}

export async function createQuizAnswer(question: KanjiChar, answer: KanjiChar, source: SourceId) {
  const response = await post('create-quiz-answer', { question, answer, source });
  const data: QuizAnswerId = await response.json();
  return data;
}

export async function createRandomQuiz() {
  const response = await get('create-random-quiz');
  const data: Quiz = await response.json();
  return data;
}

export async function createSource(source?: Option<string | string[]>) {
  if (Array.isArray(source)) {
    await Promise.all(source.map(createSource));
  }
  else if (typeof source === 'string' && source.length > 0) {
    await post('create-source', { source });
  }
}

export async function getBookmarks() {
  const response = await get('get-bookmarks');
  const data: readonly Bookmark[] = await response.json();
  return data;
}

export async function getQuizAnswers() {
  const response = await get('get-quiz-answers');
  const data: readonly QuizAnswer[] = await response.json();
  return data;
}

export async function getSet() {
  const response = await get('get-set');
  const data: KanjiSet = await response.json();
  return data;
}

export async function getSources() {
  const response = await get('get-sources');
  const data: readonly Source[] = await response.json();
  return data;
}

export async function renameSource(id: SourceId, name: string) {
  await post('rename-source', { id, name });
}

export async function removeBookmark(id: BookmarkId) {
  const response = await post('remove-bookmark', { id });
  const data: number = await response.json();
  return data;
}

export async function removeSource(id: SourceId) {
  const response = await post('remove-source', { id });
  const data: number = await response.json();
  return data;
}

export async function searchKanji() {
  const response = await get('search-kanji');
  const data: KanjiStats[] = await response.json();
  return data;
}

export async function searchSnippets(kanji: KanjiChar, source?: Option<SourceId>) {
  const response = await post('search-snippets', { kanji, source });
  const data: Snippet[] = await response.json();
  return data;
}

export async function setSourceWeight(id: SourceId, weight: SourceWeight) {
  await post('set-source-weight', { id, weight });
}

export async function toggleSource(id: SourceId, enabled: boolean) {
  await post('toggle-source', { id, enabled });
}
