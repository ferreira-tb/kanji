type Route = 'bookmarks' | 'home' | 'quiz' | 'quiz-history' | 'settings' | 'snippets' | 'sources';

interface Sorting {
  ascending: boolean;
}

type Option<T> = import('@tb-dev/utils').Option<T>;
