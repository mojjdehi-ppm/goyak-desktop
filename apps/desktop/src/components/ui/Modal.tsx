import React, { useEffect, useRef } from 'react';
import { X } from 'lucide-react';

export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: React.ReactNode;
  icon?: React.ReactNode;
  children: React.ReactNode;
  footer?: React.ReactNode;
  size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | 'full';
  showCloseButton?: boolean;
  closeOnOverlayClick?: boolean;
  closeOnEscape?: boolean;
  className?: string;
  bodyClassName?: string;
  headerClassName?: string;
  footerClassName?: string;
}

const sizeClasses = {
  sm: 'max-w-sm',
  md: 'max-w-md',
  lg: 'max-w-lg',
  xl: 'max-w-xl',
  '2xl': 'max-w-2xl',
  full: 'max-w-[95vw] h-[90vh]',
};

export const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  icon,
  children,
  footer,
  size = 'md',
  showCloseButton = true,
  closeOnOverlayClick = true,
  closeOnEscape = true,
  className = '',
  bodyClassName = '',
  headerClassName = '',
  footerClassName = '',
}) => {
  const modalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (closeOnEscape && e.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, closeOnEscape, onClose]);

  if (!isOpen) return null;

  const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (closeOnOverlayClick && e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    // Positioning is explicit rather than via daisyUI's `modal` classes: when
    // those aren't emitted, the dialog silently drops into page flow and
    // renders at the bottom of the document.
    <div
      onClick={handleOverlayClick}
      className="fixed inset-0 z-[100] bg-black/70 backdrop-blur-sm p-3 sm:p-4 transition-opacity duration-150 flex items-center justify-center"
      role="dialog"
      aria-modal="true"
    >
      <div
        ref={modalRef}
        className={`w-full max-h-[90vh] p-0 bg-base-200 border border-base-content/[0.09] rounded-3xl shadow-2xl shadow-black/60 overflow-hidden flex flex-col ${sizeClasses[size]} ${className}`}
      >
        {(title || showCloseButton) && (
          <div className={`px-6 py-4 border-b border-base-300/80 flex items-center justify-between shrink-0 bg-base-200 ${headerClassName}`}>
            <div className="flex items-center gap-1 space-x-1 rtl:space-x-reverse">
              {icon && <div className="text-base-content/80 flex items-center">{icon}</div>}
              {title && <h3 className="text-base font-bold text-white tracking-tight">{title}</h3>}
            </div>
            {showCloseButton && (
              <button
                type="button"
                onClick={onClose}
                className="btn btn-ghost btn-sm btn-square text-base-content/70 hover:text-white rounded-xl cursor-pointer"
                aria-label="Close modal"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>
        )}

        <div className={`flex-1 overflow-hidden p-0 text-base-content ${bodyClassName}`}>
          {children}
        </div>

        {footer && (
          <div className={`px-6 py-3.5 border-t border-base-300/80 flex items-center justify-between shrink-0 bg-base-200 ${footerClassName}`}>
            {footer}
          </div>
        )}
      </div>
    </div>
  );
};
