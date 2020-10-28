
(defun unicode-block-find (char)
  "Find unicode block for character CHAR.
When called interactively, character at point is used as CHAR."
  (interactive (list (char-after)))
  (let ((block (seq-find
                (lambda (sym)
                  (let ((range (symbol-value sym)))
                    (and (<= (car range) char)
                         (>= (cdr range) char))))
                unicode-blocks)))
    (message "%s" (symbol-name block))))

(provide 'unicode-block)
