import { panic } from '@tb-dev/utils';

function url(endpoint: string) {
  const base = localStorage.getItem('kanji:api-url');
  return base ? `http://${base}/${endpoint}` : panic('Missing API url.');
}

export async function get(endpoint: string) {
  const response = await fetch(url(endpoint), { method: 'GET' });
  if (response.ok) {
    return response;
  }
  else {
    throw new Error(await response.text());
  }
}

export async function post(endpoint: string, body: Record<string, unknown>) {
  const response = await fetch(url(endpoint), {
    method: 'POST',
    body: JSON.stringify(body, null, 0),
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (response.ok) {
    return response;
  }
  else {
    throw new Error(await response.text());
  }
}
