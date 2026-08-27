import React, { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { ArrowLeftRight, ChevronDown, Check } from 'lucide-react';
import { SUPPORTED_AUDIO_LANGUAGES, SupportedLanguage } from '@dubly/shared';
import { FieldLabel } from './DubbingSetupView';

interface LanguageSelectorProps {
  sourceLanguage: string;
  targetLanguage: string;
  onSourceChange: (lang: string) => void;
  onTargetChange: (lang: string) => void;
  disabled?: boolean;
}

const CustomSelect = ({
  value,
  options,
  onChange,
  disabled,
}: {
  value: string;
  options: SupportedLanguage[];
  onChange: (val: string) => void;
  disabled: boolean;
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const selected = options.find((o) => o.code === value) || options[0];

  return (
    <div className="relative" ref={containerRef}>
      <button
        type="button"
        disabled={disabled}
        onClick={() => setIsOpen(!isOpen)}
        aria-expanded={isOpen}
        className="w-full h-12 bg-base-200 border border-base-content/[0.07] hover:border-base-content/[0.14] text-base-content text-sm rounded-xl px-4 flex items-center justify-between focus:outline-none focus:border-primary transition-colors disabled:opacity-50 cursor-pointer"
      >
        <div className="flex items-center gap-2.5 rtl:space-x-reverse truncate">
          {selected.countryCode ? (
            <span className={`fi fi-${selected.countryCode} text-base rounded-[3px] shrink-0`} />
          ) : (
            <span className="text-base shrink-0">{selected.flag}</span>
          )}
          <span className="truncate font-medium">{selected.name}</span>
        </div>
        <ChevronDown
          className={`w-4 h-4 text-base-content/50 shrink-0 transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`}
        />
      </button>

      {isOpen && !disabled && (
        <div className="absolute z-50 w-full mt-2 bg-elevated border border-base-content/[0.09] rounded-xl shadow-2xl shadow-black/60 overflow-hidden py-1 max-h-60 overflow-y-auto">
          {options.map((lang) => {
            const isSelected = value === lang.code;
            return (
              <button
                key={lang.code}
                type="button"
                onClick={() => {
                  onChange(lang.code);
                  setIsOpen(false);
                }}
                className={`w-full flex items-center gap-2.5 rtl:space-x-reverse px-4 py-2.5 text-sm transition-colors cursor-pointer text-start ${
                  isSelected
                    ? 'text-primary bg-primary/10 font-medium'
                    : 'text-base-content/80 hover:bg-base-content/5 hover:text-base-content'
                }`}
              >
                {lang.countryCode ? (
                  <span className={`fi fi-${lang.countryCode} text-base rounded-[3px] shrink-0`} />
                ) : (
                  <span className="text-base shrink-0">{lang.flag}</span>
                )}
                <span className="truncate flex-1">
                  {lang.name} <span className="opacity-60">({lang.nativeName})</span>
                </span>
                {isSelected && <Check className="w-4 h-4 shrink-0" />}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
};

export const LanguageSelector: React.FC<LanguageSelectorProps> = ({
  sourceLanguage,
  targetLanguage,
  onSourceChange,
  onTargetChange,
  disabled = false,
}) => {
  const { t } = useTranslation();

  const handleSwap = () => {
    if (sourceLanguage !== 'auto') {
      const temp = sourceLanguage;
      onSourceChange(targetLanguage);
      onTargetChange(temp);
    }
  };

  return (
    <div className="space-y-2 relative z-20">
      <FieldLabel>{t('home.languages')}</FieldLabel>

      <div className="grid grid-cols-1 md:grid-cols-[1fr_auto_1fr] items-end gap-3">
        <div className="space-y-1.5">
          <span className="text-[11px] font-medium text-base-content/35 block px-1 text-start">
            {t('home.sourceLanguage')}
          </span>
          <CustomSelect
            value={sourceLanguage}
            options={SUPPORTED_AUDIO_LANGUAGES}
            onChange={onSourceChange}
            disabled={disabled}
          />
        </div>

        <button
          type="button"
          disabled={disabled || sourceLanguage === 'auto'}
          onClick={handleSwap}
          title="Swap Languages"
          aria-label="Swap Languages"
          className="h-12 w-12 rounded-xl bg-base-200 border border-base-content/[0.07] text-base-content/50 hover:text-base-content hover:border-base-content/[0.14] transition-colors disabled:opacity-30 disabled:cursor-not-allowed hidden md:flex items-center justify-center cursor-pointer"
        >
          <ArrowLeftRight className="w-4 h-4" />
        </button>

        <div className="space-y-1.5">
          <span className="text-[11px] font-medium text-base-content/35 block px-1 text-start">
            {t('home.targetLanguage')}
          </span>
          <CustomSelect
            value={targetLanguage}
            options={SUPPORTED_AUDIO_LANGUAGES.filter((l) => l.code !== 'auto')}
            onChange={onTargetChange}
            disabled={disabled}
          />
        </div>
      </div>
    </div>
  );
};
